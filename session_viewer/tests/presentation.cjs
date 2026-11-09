const fs = require('fs');
const http = require('http');
const path = require('path');
const { chromium } = require('playwright');

const root = path.resolve(__dirname, '..');
const deckDirectory = process.env.DECK_DIR || path.join(root, 'examples', 'linkwitz', 'generated');
const routes = [
  ['/deck/', deckDirectory],
  ['/', process.env.VIEWER_DIST || path.join(root, 'dist')],
];
const types = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.yaml': 'application/yaml', '.pb': 'application/octet-stream', '.ttf': 'font/ttf', '.css': 'text/css', '.json': 'application/json' };
const deck = JSON.parse(fs.readFileSync(path.join(deckDirectory, 'presentation.yaml'), 'utf8'));
const steps = deck.topics.flatMap(topic => topic.steps);
const lastStep = steps.length - 1;
const requested = [];
const server = http.createServer((request, response) => {
  const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
  if (pathname.endsWith('.pb') && !request.headers.range) requested.push(pathname);
  const route = routes.find(([prefix]) => pathname.startsWith(prefix));
  if (!route) {
    response.writeHead(404).end();
    return;
  }
  const base = path.resolve(route[1]);
  const relative = route[0] === '/' ? pathname.slice(1) : pathname.slice(route[0].length);
  const filename = pathname === '/' ? path.join(base, 'index.html') : path.resolve(base, relative);
  if (filename !== base && !filename.startsWith(base + path.sep)) {
    response.writeHead(403).end();
    return;
  }
  fs.readFile(filename, (error, data) => {
    if (error) {
      response.writeHead(404).end();
      return;
    }
    const type = types[path.extname(filename)] || 'application/octet-stream';
    const range = /^bytes=(\d+)-(\d+)$/.exec(request.headers.range || '');
    if (range) {
      const first = Number(range[1]);
      const last = Math.min(Number(range[2]), data.length - 1);
      response.writeHead(206, {
        'Content-Type': type,
        'Cache-Control': 'no-store',
        'Content-Length': last - first + 1,
        'Content-Range': `bytes ${first}-${last}/${data.length}`,
      });
      response.end(data.subarray(first, last + 1));
      return;
    }
    response.writeHead(200, { 'Content-Type': type, 'Cache-Control': 'no-store' });
    response.end(data);
  });
});

(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || 'C:/Program Files/Google/Chrome/Application/chrome.exe',
    headless: process.env.HEADLESS !== '0',
    args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan'],
  });
  const errors = [];
  const out = path.join(root, 'target', 'presentation');
  fs.mkdirSync(out, { recursive: true });
  let page;
  try {
    page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
    page.on('pageerror', error => { errors.push(error.message); console.error(error.message); });
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(`${origin}/?inspect=1&presentation=deck/presentation.yaml`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationStep === '0', { timeout: 60000 });
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', { timeout: 60000 });
    await page.waitForFunction(() => {
      const state = JSON.parse(document.querySelector('canvas').dataset.viewerInspection);
      return state.presentation_effects.active.some(effect => effect.name === 'polygon-0' && effect.morphs.length);
    }, { timeout: 60000 });
    const firstMorph = await page.evaluate(() => {
      const state = JSON.parse(document.querySelector('canvas').dataset.viewerInspection);
      return state.presentation_effects.active.find(effect => effect.name === 'polygon-0')?.morphs[0];
    });
    if (!firstMorph || firstMorph.from[0][0] >= deck.spatial_split * deck.width || firstMorph.to[0][0] <= deck.spatial_split * deck.width) {
      throw new Error(`first force does not morph from the 3D region into the polygon: ${JSON.stringify(firstMorph)}`);
    }
    const directions = await page.evaluate(() => {
      const state = JSON.parse(document.querySelector('canvas').dataset.viewerInspection);
      return state.presentation_effects.active.flatMap(effect => effect.morphs.map(morph => {
        const from = [morph.from[1][0] - morph.from[0][0], morph.from[1][1] - morph.from[0][1]];
        const to = [morph.to[1][0] - morph.to[0][0], morph.to[1][1] - morph.to[0][1]];
        const dot = from[0] * to[0] + from[1] * to[1];
        const lengths = Math.hypot(...from) * Math.hypot(...to);
        return { name: effect.name, alignment: dot / lengths, from, to };
      }));
    });
    if (directions.some(direction => direction.alignment < 0.999)) throw new Error(`force direction differs from its cable: ${JSON.stringify(directions)}`);
    const firstTrajectory = page.evaluate(() => new Promise(resolve => {
      const violations = [];
      const latest = {};
      let samples = 0;
      let started;
      const sample = now => {
        started ??= now;
        const canvas = document.querySelector('canvas');
        const state = JSON.parse(canvas.dataset.viewerInspection || '{}');
        for (const effect of state.presentation_effects?.active || []) {
          for (const morph of effect.morphs || []) {
            if (!morph.current) continue;
            const points = morph.local.map(point => [
              morph.current[0] * point[0] + morph.current[4] * point[1] + morph.current[12],
              morph.current[1] * point[0] + morph.current[5] * point[1] + morph.current[13],
            ]);
            latest[effect.name] = points;
            samples++;
            for (const [x, y] of points) {
              if (x < 20 || x > 1580 || y < 20 || y > 740) violations.push({ name: effect.name, x, y });
            }
          }
        }
        if (now - started < 20000 && (now - started < 500 || canvas.dataset.presentationAnimating !== 'false')) requestAnimationFrame(sample);
        else resolve({ violations, latest, samples });
      };
      requestAnimationFrame(sample);
    }));
    await page.waitForTimeout(600);
    const moving = await page.evaluate(() => {
      const state = JSON.parse(document.querySelector('canvas').dataset.viewerInspection);
      return state.presentation_effects.active.find(effect => effect.name === 'polygon-0')?.morphs[0];
    });
    const length = endpoints => Math.hypot(endpoints[1][0] - endpoints[0][0], endpoints[1][1] - endpoints[0][1]);
    const expectedScale = length(moving.from) / length(moving.to);
    const currentScale = Math.hypot(...moving.current.slice(0, 3));
    if (Math.abs(currentScale - expectedScale) > expectedScale * 0.05) {
      throw new Error(`force shortened before traveling: scale ${currentScale}, expected ${expectedScale}`);
    }
    await page.screenshot({ path: path.join(out, 'morph-mid.png') });
    const firstAnimation = await page.evaluate(() => new Promise(resolve => {
      const intervals = [];
      let previous;
      let started;
      const sample = now => {
        started ??= now;
        if (previous !== undefined) intervals.push(now - previous);
        previous = now;
        if (now - started < 2200) requestAnimationFrame(sample);
        else {
          const sorted = intervals.slice().sort((a, b) => a - b);
          const mean = intervals.reduce((sum, value) => sum + value, 0) / intervals.length;
          resolve({ frames: intervals.length, meanMs: mean, p95Ms: sorted[Math.floor(sorted.length * 0.95)], maxMs: Math.max(...intervals) });
        }
      };
      requestAnimationFrame(sample);
    }));
    console.log('first animation timing', JSON.stringify(firstAnimation));
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 60000 });
    const trajectory = await firstTrajectory;
    if (trajectory.samples < 100 || trajectory.violations.length) throw new Error(`first-slide trajectory failed: ${JSON.stringify(trajectory.violations.slice(0, 8))}`);
    for (let index = 0; index < 5; index++) {
      const current = trajectory.latest[`polygon-${index}`];
      const next = trajectory.latest[`polygon-${(index + 1) % 5}`];
      if (!current || !next || Math.hypot(current[1][0] - next[0][0], current[1][1] - next[0][1]) > 5) throw new Error(`force polygon is not closed at ${index}: ${JSON.stringify(trajectory.latest)}`);
    }
    console.log(`first trajectory: ${trajectory.samples} on-canvas morph samples; closed polygon`);
    const settled = JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
    const projected = (matrix, point) => {
      const clip = [0, 1, 2, 3].map(row => matrix[row] * point[0] + matrix[row + 4] * point[1] + matrix[row + 8] * point[2] + matrix[row + 12]);
      return [(0.5 + clip[0] / clip[3] * 0.5) * settled.canvas[0], (0.5 - clip[1] / clip[3] * 0.5) * settled.canvas[1]];
    };
    const actual = [];
    for (let index = 0; index < 5; index++) {
      const effect = settled.presentation_effects.effects.find(effect => effect.name === `polygon-${index}`);
      if (!effect || effect.endpoints.length !== 1 || effect.source_endpoints?.length !== 1) {
        throw new Error(`missing rendered force endpoints for polygon-${index}: ${JSON.stringify(effect)}`);
      }
      const source = effect.source_endpoints[0].map(point => projected(settled.spatial_view.matrix, point.map((value, axis) => value + settled.origin[axis])));
      const target = effect.endpoints[0].map(point => projected(settled.mvp, point));
      const from = [source[1][0] - source[0][0], source[1][1] - source[0][1]];
      const to = [target[1][0] - target[0][0], target[1][1] - target[0][1]];
      const alignment = (from[0] * to[0] + from[1] * to[1]) / (Math.hypot(...from) * Math.hypot(...to));
      if (alignment < 0.999) throw new Error(`rendered polygon-${index} differs from its projected cable: ${JSON.stringify({ source, target, alignment })}`);
      for (const [x, y] of target) {
        if (x < 20 || x > settled.canvas[0] - 20 || y < 20 || y > settled.canvas[1] - 20) throw new Error(`rendered polygon-${index} is off canvas: ${JSON.stringify(target)}`);
      }
      actual.push({ source, target, alignment });
      const next = settled.presentation_effects.effects.find(effect => effect.name === `polygon-${(index + 1) % 5}`);
      const nextStart = projected(settled.mvp, next.endpoints[0][0]);
      if (Math.hypot(target[1][0] - nextStart[0], target[1][1] - nextStart[1]) > 5) throw new Error(`rendered polygon is not closed at ${index}`);
    }
    console.log('rendered force alignment', JSON.stringify(actual));
    await page.locator('canvas').focus();
    if (await page.locator('#no-webgpu').count()) throw new Error('WebGPU is unavailable');
    if (await page.locator('#viewer-error:not([hidden])').count()) throw new Error(await page.locator('#viewer-error-message').textContent());
    if (await page.locator('button').evaluateAll(elements => elements.filter(element => element.getClientRects().length).length)) throw new Error('unexpected visible controls');
    const inspection = async (settled = false) => {
      await page.waitForFunction(settled => {
        const canvas = document.querySelector('canvas');
        const state = JSON.parse(canvas?.dataset.viewerInspection || '{}');
        return canvas?.dataset.presentationReady === 'true'
          && state.presentation_effects?.step === Number(canvas?.dataset.presentationStep)
          && (!settled || canvas?.dataset.presentationAnimating === 'false');
      }, settled);
      return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
    };
    const expression = async name => {
      const state = await inspection();
      return state.text_labels.filter(label => label.name === name).map(label => label.text).join('').normalize('NFKC').replace(/\s/g, '');
    };
    const centerOf = (state, name) => {
      const effect = state.presentation_effects.effects.find(effect => effect.name === name);
      if (!effect || effect.centers.length !== 1) throw new Error(`missing geometry center for ${name}: ${JSON.stringify(state.presentation_effects)}`);
      return effect.centers[0];
    };
    const project = (state, at) => {
      const m = state.spatial_view.matrix;
      const p = at.map((value, axis) => value - state.origin[axis]);
      const clip = [0, 1, 2, 3].map(row => m[row] * p[0] + m[4 + row] * p[1] + m[8 + row] * p[2] + m[12 + row]);
      return [(0.5 + clip[0] / clip[3] * 0.5) * state.canvas[0], (0.5 - clip[1] / clip[3] * 0.5) * state.canvas[1]];
    };
    const screenshot = async index => {
      await page.screenshot({ path: path.join(out, `step-${String(index).padStart(2, '0')}.png`) });
    };
    const title = await page.locator('#presentation-topic').textContent();
    if (title.includes('·')) throw new Error('title still contains a center dot');
    if (!(await page.locator('#presentation-topic').evaluate(element => getComputedStyle(element).textAlign === 'center' && element.style.left === '50%'))) throw new Error('title is not centered');
    const sum = await expression('equilibrium-sum');
    if (!sum.includes('∑') || !sum.includes('=0')) throw new Error(`equilibrium lacks its summation: ${sum}`);
    if (await expression('polygon-label-0') !== 'f0') throw new Error('first force label did not reach the polygon');
    const initial = await inspection();
    const source = initial.presentation_effects.spatial.rows.find(([name]) => name === 'e0');
    await page.mouse.click(...project(initial, source[1].slice(0, 3)));
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationLinked.includes('e0'), null, { timeout: 5000 });
    await page.keyboard.press('Escape');
    const firstImage = await page.screenshot({ path: path.join(out, 'step-00.png') });
    const freeNode = initial.presentation_effects.spatial.rows.find(([name]) => name === 'n0');
    const freePixel = project(initial, freeNode[1].slice(0, 3)).map(Math.round);
    const pinkPixels = await page.evaluate(async ({ encoded, at }) => {
      const image = new Image();
      image.src = `data:image/png;base64,${encoded}`;
      await image.decode();
      const canvas = document.createElement('canvas');
      canvas.width = image.width;
      canvas.height = image.height;
      const context = canvas.getContext('2d');
      context.drawImage(image, 0, 0);
      const pixels = context.getImageData(at[0] - 8, at[1] - 8, 17, 17).data;
      let count = 0;
      for (let index = 0; index < pixels.length; index += 4) {
        if (Math.abs(pixels[index] - 220) <= 3 && Math.abs(pixels[index + 1] - 75) <= 3 && Math.abs(pixels[index + 2] - 141) <= 3) count++;
      }
      return count;
    }, { encoded: firstImage.toString('base64'), at: freePixel });
    if (!pinkPixels) throw new Error(`free node is not #dc4b8d at ${freePixel}`);
    for (let index = 1; index <= lastStep; index++) {
      await page.keyboard.press('ArrowRight');
      await page.waitForFunction(expected => document.querySelector('canvas')?.dataset.presentationStep === String(expected), index, { timeout: 30000 });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', { timeout: 30000 });
      if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') throw new Error(`text overlap on the first frame of reveal ${index}`);
      if (index === 11) {
        const q2Start = await inspection();
        if (Math.abs(centerOf(q2Start, 'line-n0')[0] - 5) > 1e-6) throw new Error('q2 case did not start at five');
        const weighted = await expression('weighted');
        if (!weighted.includes('q1+q2') || !weighted.includes('q1x1+q2x2+p')) throw new Error(`weighted-average fraction is missing: ${weighted}`);
      }
      if (index === 12) {
        await page.waitForFunction(() => {
          const state = JSON.parse(document.querySelector('canvas')?.dataset.viewerInspection || '{}');
          const effect = state.presentation_effects?.effects.find(effect => effect.name === 'line-n0');
          return effect && effect.centers[0][0] > 5.01 && effect.centers[0][0] < 6.99;
        }, null, { timeout: 15000 });
      }
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 30000 });
      if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') throw new Error(`text overlap after reveal ${index}`);
      const textStatus = await page.locator('#viewer-status').textContent();
      if (textStatus.startsWith('Text:')) throw new Error(`step ${index}: ${textStatus}`);
      await screenshot(index);
      if (index === 2) {
        if (await expression('coords-1')) throw new Error('3D coordinate tuple is still visible');
        if (await expression('load-coordinate')) throw new Error('3D load coordinate is still visible');
        for (const file of ['00_00', '01_00']) {
          const count = requested.filter(name => name.endsWith(`/deck/steps/${file}.pb`)).length;
          if (count !== 1) throw new Error(`${file} was requested ${count} times instead of once`);
        }
      }
      if (index === 2) {
        const state = await inspection();
        const first = state.text_labels.find(label => label.name === 'C[0,0]');
        const below = state.text_labels.find(label => label.name === 'C[1,0]');
        if (!first || !below) throw new Error('matrix values are missing');
        if (Math.abs(below.world[0] - first.world[0]) > 1e-6 || Math.abs(below.world[1] - first.world[1] + 80) > 1e-6) throw new Error('matrix values are not on their grid');
        await page.evaluate(() => window.wasmBindings.presentation_named('C[1,2]', true));
        await page.waitForFunction(() => {
          const linked = document.querySelector('canvas')?.dataset.presentationLinked.split(',') || [];
          return linked.includes('e1') && linked.includes('C');
        });
        await page.keyboard.press('Escape');
        const rightBefore = await page.screenshot({ clip: { x: 830, y: 120, width: 580, height: 500 } });
        const beforeOrbit = (await inspection()).spatial_view.matrix;
        await page.locator('canvas').focus();
        await page.mouse.move(400, 470);
        await page.waitForTimeout(150);
        await page.mouse.down({ button: 'right' });
        await page.mouse.move(520, 420, { steps: 12 });
        await page.mouse.up({ button: 'right' });
        await page.waitForTimeout(350);
        const afterOrbit = (await inspection()).spatial_view.matrix;
        if (JSON.stringify(beforeOrbit) === JSON.stringify(afterOrbit)) {
          console.error('orbit inspection', JSON.stringify({ beforeOrbit, afterOrbit, interacting: (await inspection()).interacting }));
          throw new Error('right drag did not orbit');
        }
        const rightAfter = await page.screenshot({ clip: { x: 830, y: 120, width: 580, height: 500 } });
        if (!rightBefore.equals(rightAfter)) throw new Error('orbit changed the 2D mathematics');
        if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') throw new Error('text overlap during orbit');
        await page.mouse.move(430, 460);
        await page.mouse.wheel(0, -320);
        await page.waitForTimeout(350);
        if (JSON.stringify(afterOrbit) === JSON.stringify((await inspection()).spatial_view.matrix)) throw new Error('wheel did not zoom');
        if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') throw new Error('text overlap during zoom');
      }
      if (index === 3) {
        const state = await inspection();
        if (state.presentation_effects.spatial.rows.some(([name]) => name.startsWith('angle-'))) throw new Error('misleading angle arcs are still present');
      }
      if (index === 8) {
        const state = await inspection();
        if (state.presentation_effects.spatial.rows.some(([name]) => name.startsWith('box-'))) throw new Error('minimal x projection retained construction boxes');
      }
      if (index === 11) {
        await page.keyboard.press('ArrowLeft');
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationStep === '10');
        await page.keyboard.press('ArrowRight');
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationStep === '11');
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true');
        await page.waitForFunction(() => {
          const state = JSON.parse(document.querySelector('canvas')?.dataset.viewerInspection || '{}');
          const effect = state.presentation_effects?.effects.find(effect => effect.name === 'line-n0');
          return effect && effect.centers[0][0] > 5.01 && effect.centers[0][0] < 7.49;
        }, null, { timeout: 15000 });
        if (await page.locator('canvas').getAttribute('data-presentation-animating') !== 'true') throw new Error('q2 animation is too short to inspect');
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 30000 });
        const end = await inspection();
        if (Math.abs(centerOf(end, 'line-n0')[0] - 7.5) > 1e-6) throw new Error('q2 case did not end at 7.5');
        for (const [name, expected] of [['line-cable1', 3.75], ['line-cable2', 8.75], ['line-q1', 7.1], ['line-q2', 8.7]]) {
          const center = centerOf(end, name);
          if (Math.abs(center[0] - expected) > 1e-6 || Math.abs(center[1] + 2.5) > 1e-6) throw new Error(`${name} is not attached to the 1D axis: ${center}`);
        }
        for (const name of ['line-q1-label', 'line-q2-solved']) {
          const label = end.text_labels.find(label => label.name === name);
          if (!label || label.world[1] <= -2.5) throw new Error(`${name} is not above the 1D cable`);
        }
        if (await expression('line-q2-solved') !== 'q2=3') throw new Error('q2 did not become three');
        if (!(await expression('case-1-solved')).endsWith('=7.5')) throw new Error('q2 substitution did not solve');
        if (!(await expression('line-lengths-forces')).includes('f1=f2=7.5')) throw new Error('stronger-neighbour forces are not explained');
        await screenshot(11);
      }
      if (index === 12) {
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 30000 });
        const end = await inspection();
        if (Math.abs(centerOf(end, 'line-n0')[0] - 7) > 1e-6) throw new Error('external-load case did not end at seven');
        for (const [name, expected] of [['line-cable1', 3.5], ['line-cable2', 8.5], ['line-q1', 6.6], ['line-q2', 7.4], ['line-p', 6.4]]) {
          const center = centerOf(end, name);
          if (Math.abs(center[0] - expected) > 1e-6 || Math.abs(center[1] + 2.5) > 1e-6) throw new Error(`${name} is not attached to the loaded 1D axis: ${center}`);
        }
        if (await expression('line-x0-solved') !== 'x0=7') throw new Error('external load did not update the x0 label');
        if (!(await expression('case-2-solved')).endsWith('=7')) throw new Error('external-load substitution did not solve');
        await screenshot(12);
      }
      if (index === 13) {
        const state = await inspection();
        const moved = state.presentation_effects.spatial.rows.find(([name]) => name === 'n0-moved');
        if (!moved) throw new Error('moved point is missing');
        const center = moved[1].slice(0, 3);
        if (center.some((value, axis) => Math.abs(value - [3, 3.7, 0.85][axis]) > 1e-6)) throw new Error('pulse moved its anchor');
        const at = project(state, center);
        await page.mouse.click(...at);
        await page.waitForTimeout(300);
        const linked = await page.locator('canvas').getAttribute('data-presentation-linked');
        if (!linked.includes('n0-moved') || !linked.includes('L')) throw new Error(`pulse is not anchored at its point: ${linked}`);
        await page.keyboard.press('Escape');
      }
      if (index >= 19 && index <= 22) {
        const expected = index === 19 ? ['Cx-C', 'Cx-x', 'Cx-u'] : index === 20 ? ['CN', 'xN', 'CNxN'] : index === 21 ? ['CF', 'xF', 'CFxF'] : ['CNxN', 'CFxF', 'u'];
        for (const name of expected) {
          if (!(await expression(`${name}-title`))) throw new Error(`missing matrix title for ${name}`);
        }
        const state = await inspection();
        const flashes = state.presentation_effects.effects.filter(effect => effect.kind === 'Highlight');
        if (!flashes.length) throw new Error(`matrix calculation ${index} has no cell flashes`);
        if (index === 22) {
          const check = await expression('u-sum-3-0');
          if (!check.includes('3-7=-4')) throw new Error(`final coordinate-difference sum is missing: ${check}`);
        }
      }
      if (index === 16) {
        if ((await expression('U[0,0]')) !== '3') throw new Error('diagonal value did not use the fitted automatic label');
        if (!(await expression('Linv[0,0]')).includes('0.24')) throw new Error('inverse-length value is not rounded and fitted');
      }
      if (index === 18) {
        if (!(await expression('q-equilibrium')).includes('Qu')) throw new Error('in-place substitution did not reach the final equilibrium');
        if (!(await expression('q-no-length')).includes('nolengthsremain.')) throw new Error('length cancellation is not explained');
      }
      if (index >= 26 && index <= 30) {
        const letter = String.fromCharCode(97 + index - 26);
        const symbolic = await expression(`node-symbolic-${letter}`);
        const solved = await expression(`node-solved-${letter}`);
        const calculation = await expression(`node-rhs-${index - 26}-0-calculation`);
        if (!symbolic.includes('x') || !solved.startsWith(`x${letter}=`)) throw new Error(`node ${letter} derivation is incomplete`);
        if (!calculation.includes('×')) throw new Error(`node ${letter} does not show its expanded dot product`);
        if (!(await expression(`node-x-${letter}`)).startsWith(`x${letter}`)) throw new Error(`node ${letter} lacks its coordinate arrow label`);
      }
      if (index === 26) {
        const state = await inspection();
        const nodes = state.presentation_effects.spatial.rows.filter(([name]) => name.startsWith('net-node-'));
        const cables = state.presentation_effects.spatial.rows.filter(([name]) => name.startsWith('net-edge-'));
        if (nodes.length !== 9 || cables.length !== 12) throw new Error(`next duplicated the small net: ${nodes.length} nodes, ${cables.length} cables`);
        if (state.presentation_effects.effects.some(effect => effect.kind === 'Grow' && effect.name.startsWith('net-node-'))) throw new Error('next regrew the small net');
        if (await expression('net-system')) throw new Error('previous small-net mathematics remained visible');
        if (await expression('net-diagonal')) throw new Error('previous small-net annotations remained visible');
        if ((await expression('net-stiffness-title')) !== 'A') throw new Error('previous small-net matrix remained visible');
      }
      if (index === 32 && !(await expression('coordinate-e')).startsWith('e:')) {
        throw new Error('coordinate rows are not mapped back to their nodes');
      }
      if (index === 25 || index === 33) {
        const state = await inspection();
        const center = state.presentation_effects.spatial.rows.find(([name]) => name === 'net-node-4')?.[1].slice(0, 3);
        if (!center || center.every((value, axis) => Math.abs(value - [1, 1, 0][axis]) < 1e-9)) throw new Error('small net is still in its initial, unrelaxed form');
        await page.evaluate(() => window.wasmBindings.presentation_named('net-stiffness[2,2]', true));
        await page.waitForTimeout(250);
        const linked = await page.locator('canvas').getAttribute('data-presentation-linked');
        if (!linked.includes('net-node-4') || !linked.includes('net-stiffness')) throw new Error(`net matrix link failed: ${linked}`);
        await page.keyboard.press('Escape');
      }
      if (index === 34) {
        if (!(await expression('large-z')).includes('=-0.12')) throw new Error('local vertical equilibrium is missing');
        if (!(await expression('large-z-average')).includes('+0.03')) throw new Error('local averaging rule is missing');
        const state = await inspection();
        if (!state.presentation_effects.spatial.rows.some(([name]) => name === 'net-node-24')) throw new Error('local interior node is missing');
      }
      if (index === 35) {
        const formula = await expression('large-force');
        if (!formula.includes('0.88') || !formula.includes('kN')) throw new Error(`force recovery formula is missing: ${formula}`);
        const state = await inspection();
        const nodes = state.presentation_effects.spatial.rows.filter(([name]) => name.startsWith('net-node-'));
        const cables = state.presentation_effects.spatial.rows.filter(([name]) => name.startsWith('net-edge-'));
        if (nodes.length !== 49 || cables.length !== 84) throw new Error(`next duplicated the large net: ${nodes.length} nodes, ${cables.length} cables`);
        if (state.presentation_effects.effects.some(effect => effect.kind === 'Grow' && effect.name.startsWith('net-node-'))) throw new Error('next regrew the large net');
        const linkedName = deck.links.find(group => group.includes('large-force'))?.[0];
        await page.evaluate(name => window.wasmBindings.presentation_named(name, true), linkedName);
        await page.waitForTimeout(250);
        const linked = await page.locator('canvas').getAttribute('data-presentation-linked');
        if (!linked.includes(linkedName) || !linked.includes('large-force')) throw new Error('force equation is not linked to its cable');
        await page.keyboard.press('Escape');
      }
      if (index === 33) {
        const state = await inspection();
        const flashes = state.presentation_effects.effects.filter(effect => effect.kind === 'Highlight');
        if (flashes.length !== 33) throw new Error(`final XYZ verification has ${flashes.length} instead of 33 cell flashes`);
      }
    }
    await page.keyboard.press('ArrowRight');
    await page.waitForTimeout(200);
    if (await page.locator('canvas').getAttribute('data-presentation-step') !== String(lastStep)) throw new Error('next passed the last step');
    for (let index = lastStep - 1; index >= 0; index--) {
      await page.keyboard.press('ArrowLeft');
      await page.waitForFunction(expected => document.querySelector('canvas')?.dataset.presentationStep === String(expected), index, { timeout: 30000 });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', { timeout: 30000 });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 30000 });
    }
    if (await page.locator('canvas').getAttribute('data-presentation-topic') !== '0') throw new Error('back did not restore the first topic');
    await page.keyboard.press('ArrowLeft');
    await page.waitForTimeout(200);
    if (await page.locator('canvas').getAttribute('data-presentation-step') !== '0') throw new Error('previous passed the first step');
    if (errors.length) throw new Error(errors.join('\n'));
    console.log(JSON.stringify({ ok: true, steps: steps.length, topics: deck.topics.length, firstAnimation, screenshots: out }));
  } finally {
    await browser.close();
    await new Promise(resolve => server.close(resolve));
  }
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
