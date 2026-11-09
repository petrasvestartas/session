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
const server = http.createServer((request, response) => {
  const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
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
    response.writeHead(200, { 'Content-Type': types[path.extname(filename)] || 'application/octet-stream', 'Cache-Control': 'no-store' });
    response.end(data);
  });
});

const project = (state, at) => {
  const m = state.spatial_view.matrix;
  const p = at.map((value, axis) => value - state.origin[axis]);
  const clip = [0, 1, 2, 3].map(row => m[row] * p[0] + m[4 + row] * p[1] + m[8 + row] * p[2] + m[12 + row]);
  return [(0.5 + clip[0] / clip[3] * 0.5) * state.canvas[0], (0.5 - clip[1] / clip[3] * 0.5) * state.canvas[1]];
};

const dashTransitions = async (page, state, screenshot) => {
  const a = project(state, [3, 3, 0.25]);
  const b = project(state, [3, 0, 0.25]);
  const points = Array.from({ length: 181 }, (_, index) => {
    const t = index / 180;
    return [Math.round(a[0] + (b[0] - a[0]) * t), Math.round(a[1] + (b[1] - a[1]) * t)];
  });
  return page.evaluate(async ({ encoded, points }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${encoded}`;
    await image.decode();
    const canvas = document.createElement('canvas');
    canvas.width = image.width;
    canvas.height = image.height;
    const context = canvas.getContext('2d');
    context.drawImage(image, 0, 0);
    const pixels = context.getImageData(0, 0, image.width, image.height).data;
    const lit = points.map(([x, y]) => {
      const at = (y * image.width + x) * 4;
      return pixels[at] + pixels[at + 1] + pixels[at + 2] > 180;
    });
    let transitions = 0;
    for (let index = 1; index < lit.length; index++) {
      if (lit[index] !== lit[index - 1]) transitions++;
    }
    return transitions;
  }, { encoded: screenshot.toString('base64'), points });
};

(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || 'C:/Program Files/Google/Chrome/Application/chrome.exe',
    headless: process.env.HEADLESS !== '0',
    args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan'],
  });
  const out = path.join(root, 'target', 'presentation');
  fs.mkdirSync(out, { recursive: true });
  const cases = [
    { width: 927, height: 763, dpr: 1 },
    { width: 1035, height: 766, dpr: 1 },
    { width: 943, height: 682, dpr: 1 },
    { width: 1440, height: 900, dpr: 2 },
  ];
  try {
    for (const item of cases) {
      const page = await browser.newPage({ viewport: { width: item.width, height: item.height }, deviceScaleFactor: item.dpr });
      const errors = [];
      page.on('pageerror', error => errors.push(error.message));
      page.on('console', message => {
        if (message.type() === 'error') errors.push(message.text());
        if (message.type() === 'warning' && message.text().includes('overlap')) console.log(message.text());
      });
      const tag = `${item.width}x${item.height}-dpr${item.dpr}`;
      await page.goto(`${origin}/?inspect=1&presentation=deck/presentation.yaml`, { waitUntil: 'domcontentloaded' });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationStep === '0', { timeout: 60000 });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', { timeout: 60000 });
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 60000 });
      await page.waitForTimeout(700);
      await page.locator('canvas').focus();
      for (const target of [0, 2, 3, 8, 9, 11, 12, 13, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35]) {
        console.log('quality', tag, 'reveal', target);
        while (Number(await page.locator('canvas').getAttribute('data-presentation-step')) < target) {
          const current = Number(await page.locator('canvas').getAttribute('data-presentation-step'));
          await page.keyboard.press('ArrowRight');
          await page.waitForFunction(expected => document.querySelector('canvas')?.dataset.presentationStep === String(expected), current + 1, { timeout: 30000 });
          await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', { timeout: 30000 });
        }
        await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', { timeout: 30000 });
        await page.waitForTimeout(500);
        if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') {
          await page.screenshot({ path: path.join(out, `quality-${tag}-${target}-overlap.png`) });
          throw new Error(`${tag}: label or geometry overlap at reveal ${target}`);
        }
        const state = JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
        if (Math.abs(state.spatial_view.matrix[15] - 1.0) > 1e-9) throw new Error(`${tag}: presentation is not orthographic`);
        const screenshot = await page.screenshot({ path: path.join(out, `quality-${tag}-${target}.png`) });
        if (target === 0) {
          const sum = state.text_labels.filter(label => label.name === 'equilibrium-sum').map(label => label.text).join('');
          if (!sum.includes('∑')) throw new Error(`${tag}: equilibrium lacks its summation`);
        }
        if (target === 2) {
          if (state.text_labels.some(label => label.name.startsWith('C-column') || label.name.startsWith('C-row'))) {
            throw new Error(`${tag}: matrix headers are still present`);
          }
          await page.setViewportSize({ width: item.height, height: item.width });
          await page.waitForTimeout(700);
          if (await page.locator('canvas').getAttribute('data-presentation-overlaps') !== '0') {
            throw new Error(`${tag}: resize caused label overlap`);
          }
          await page.screenshot({ path: path.join(out, `quality-${tag}-resized.png`) });
          await page.setViewportSize({ width: item.width, height: item.height });
          await page.waitForTimeout(700);
        }
        if (target === 3) {
          const transitions = await dashTransitions(page, state, screenshot);
          if (transitions < 8) throw new Error(`${tag}: only ${transitions} dash/gap transitions`);
        }
        if (target === 8 && state.presentation_effects.spatial.rows.some(([name]) => name.startsWith('box-'))) {
          throw new Error(`${tag}: minimal x projection retained construction boxes`);
        }
        if (target === 8) {
          const a = project(state, [0, -2.5, 0]);
          const b = project(state, [10, -2.5, 0]);
          if (Math.abs(a[1] - b[1]) > 1e-6 || b[0] <= a[0]) throw new Error(`${tag}: the x axis is not horizontal`);
        }
        if (target === 9) {
          const equation = state.text_labels.filter(label => label.name === 'length').map(label => label.text).join('');
          if (equation.includes('√')) throw new Error(`${tag}: radical still uses a scaled glyph`);
          if (!equation.includes('𝑙')) throw new Error(`${tag}: length equation is missing`);
        }
        if (target === 11 || target === 12) {
          const expected = target === 11 ? 7.5 : 7;
          const effect = state.presentation_effects.effects.find(effect => effect.name === 'line-n0');
          if (!effect || Math.abs(effect.centers[0][0] - expected) > 1e-6) {
            throw new Error(`${tag}: wrong 1D endpoint at reveal ${target}`);
          }
        }
        if (target === 34) {
          const equation = state.text_labels.filter(label => label.name === 'large-z-average').map(label => label.text).join('');
          if (!equation.includes('0.03')) throw new Error(`${tag}: local averaging equation is missing`);
        }
      }
      if (errors.length) throw new Error(`${tag}: ${errors.join('\n')}`);
      await page.close();
    }
    console.log(JSON.stringify({ ok: true, cases: cases.length, screenshots: out }));
  } finally {
    await browser.close();
    await new Promise(resolve => server.close(resolve));
  }
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
