const assert = require('node:assert/strict');
module.exports = async page => {
    const errors = [];
    page.on('pageerror', error => errors.push(String(error)));
    const url = new URL(page.url());
    url.searchParams.set('inspect', '1');
    url.searchParams.set('nogrid', '1');
    if (page.url() !== url.href) await page.goto(url.href);
    const state = () => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
    const ui = () => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')));
    const wait = async (key, value) => page.waitForFunction(([key, value]) => {
        const data = document.querySelector('canvas').getAttribute('data-viewer-inspection');
        return data && JSON.parse(data)[key] === value;
    }, [key, value]);
    await wait('objects', 5);
    assert.deepEqual((await state()).text_labels.map(label => label.text), ['original pedestal']);
    assert.equal((await state()).source_cpu_known_payload.unique_geometry_values, 4);
    assert.equal((await ui()).framework, 'egui 0.34.3');
    const answers = [
        [/^Select All$/, /^\d+ selected$/],
        [/^Select By Name /, /^\d+ selected: names containing/],
        [/^Line /, /^Created and selected line /],
        [/^Element Beam /, /^Created beam from its curves/],
        [/^Length BoundingBox$/, /^Length BoundingBox · X 1000 · Y 100 · Z 200 mm$/],
        [/^BoundingBox$/, /^Created and selected bounding box /],
        [/^Layers /, /^Layers \(On Off All\)$/],
        [/^View /, /^View updated$/],
        [/^Opacity /, /^Opacity 1$/],
        [/^Arctic /, /^Arctic On$/],
        [/^Fit$/, /^fitted$/],
        [/^Element Attributes (On|Off)$/, /^Element Attributes (On|Off)$/],
        [/^Element Interactions (On|Off)$/, /^Element Interactions (On|Off)$/],
    ];
    const command = async (line, answer = answers.find(([pattern]) => pattern.test(line))?.[1]) => {
        assert(answer, `Declare the expected answer for ${line}`);
        console.log(`Command: ${line}`);
        const historyLength = (await ui()).history.length;
        const control = (await ui()).controls.find(control => control.key === 'command/input');
        assert(control, 'The maintained command field must be inspected');
        const [left, top, right, bottom] = control.rect;
        await page.mouse.click((left + right) / 2, (top + bottom) / 2);
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command_open);
        if ((await ui()).command) await page.keyboard.press('ControlOrMeta+A');
        await page.keyboard.type(line);
        await page.waitForFunction(line => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command === line, line);
        await page.keyboard.press('Enter');
        await page.waitForFunction(length => {
            const data = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui'));
            return data.command === '' && data.history.length > length;
        }, historyLength);
        const result = (await ui()).history.at(-1).split('\n').slice(1).join('\n');
        assert.match(result, answer, `${line} must report its actual expected outcome`);
    };
    const canvasFocus = async () => {
        assert.equal((await ui()).command, '', 'Clear command text before leaving the field');
        const [left, top] = (await ui()).scene_rect;
        await page.mouse.move(left + 24, top + 24);
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        await page.mouse.down({button: 'middle'});
        await page.mouse.up({button: 'middle'});
        await page.locator('canvas').focus();
        await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command_open);
    };
    assert.equal((await state()).element_attributes, false);
    assert.equal((await state()).element_interactions, true);
    for (const [name, key] of [['Element Attributes', 'element_attributes'], ['Element Interactions', 'element_interactions']]) {
        for (const shown of [true, false]) {
            await command(`${name} ${shown ? 'On' : 'Off'}`);
            await wait(key, shown);
        }
    }
    await command('Element Interactions On');
    await command('Select All');
    await wait('selected_group_count', 5);
    const field = (await ui()).controls.find(control => control.key === 'command/input');
    const [left, top, right, bottom] = field.rect;
    await page.mouse.click((left + right) / 2, (top + bottom) / 2);
    await page.keyboard.type('Hide');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command === 'Hide');
    assert.equal((await state()).hidden_count, 0, 'H typed in the command field must not hide geometry');
    await page.keyboard.press('ControlOrMeta+A');
    await page.keyboard.press('Backspace');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command === '');
    await canvasFocus();
    assert.equal((await state()).selected_group_count, 5, 'Changing keyboard focus must preserve the selection');
    await page.keyboard.press('h');
    await wait('hidden_count', 5);
    await page.keyboard.press('s');
    await wait('hidden_count', 0);
    await page.keyboard.press('f');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1) === '> Fit\nfitted');
    assert((await state()).mvp.every(Number.isFinite));
    await command('Line 0,0,0 1000,0,0');
    await wait('objects', 6);
    await wait('selected_kind', 'Line');
    await command('Element Beam 100 200');
    await wait('selected_kind', 'Element');
    const element = await state();
    assert(element.objects > 5, 'The new Element and its owned attributes must be present');
    await canvasFocus();
    await page.keyboard.press('Delete');
    await wait('objects', 5);
    await page.keyboard.press('ControlOrMeta+z');
    await wait('objects', element.objects);
    await page.keyboard.press('ControlOrMeta+y');
    await wait('objects', 5);
    await page.keyboard.press('ControlOrMeta+z');
    await wait('objects', element.objects);
    await command('Select By Name beam');
    await wait('selected_kind', 'Element');
    assert.deepEqual((await state()).selected_rows, [(await state()).selected]);
    assert.deepEqual((await state()).identity, element.identity, 'Undo must restore the original Element identity');
    const before = await state();
    await command('Length BoundingBox');
    await page.waitForFunction(() => /^X 1000 · Y 100 · Z 200 /.test(JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).mark?.label || ''));
    assert.equal((await state()).undo_depth, before.undo_depth);
    await command('BoundingBox');
    await wait('objects', element.objects + 1);
    await wait('selected_kind', 'OBB');
    const boxField = (await ui()).controls.find(control => control.key === 'command/input');
    const [bl, bt, br, bb] = boxField.rect;
    await page.mouse.click((bl + br) / 2, (bt + bb) / 2);
    await page.keyboard.type('abc');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command.length > 0);
    const typed = (await ui()).command;
    await page.keyboard.press('ControlOrMeta+z');
    await page.waitForFunction(typed => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command !== typed, typed);
    assert.equal((await state()).objects, element.objects + 1, 'Undo belongs to nonempty command text');
    if ((await ui()).command) {
        await page.keyboard.press('ControlOrMeta+A');
        await page.keyboard.press('Backspace');
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).command === '');
    }
    await canvasFocus();
    await page.keyboard.press('Delete');
    await wait('objects', element.objects);
    await page.keyboard.press('ControlOrMeta+z');
    await wait('objects', element.objects + 1);
    await page.keyboard.press('ControlOrMeta+Shift+z');
    await wait('objects', element.objects);
    await command('Layers On');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).layers_open);
    const originalViewport = page.viewportSize();
    const wideViewport = {...originalViewport, width: Math.max(1440, originalViewport.width)};
    await page.setViewportSize(wideViewport);
    const width = wideViewport.width;
    await page.waitForFunction(width => {
        const data = document.querySelector('canvas').getAttribute('data-viewer-ui');
        return data && Math.abs(width - JSON.parse(data).scene_rect[2] - 413.33334) < 16;
    }, width);
    const panelWidth = width - (await ui()).scene_rect[2];
    assert(Math.abs(panelWidth - 413.33334) < 16, 'A wide viewport must expose the one-third larger default');
    await command('Layers Off');
    await page.setViewportSize(originalViewport);
    await page.waitForFunction(({width, height}) => {
        const canvas = document.querySelector('canvas');
        return canvas.width === Math.round(width * devicePixelRatio)
            && canvas.height === Math.round(height * devicePixelRatio);
    }, originalViewport);
    for (const line of ['Layers Off', 'View Isometric', 'Opacity 1', 'Arctic On', 'View Outline On', 'Fit']) await command(line);
    const start = await state();
    await canvasFocus();
    await page.mouse.move(300, 220);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(380, 250, {steps: 12});
    await page.mouse.up({button: 'right'});
    await page.waitForFunction(mvp => {
        const now = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection'));
        return now.mvp.some((value, index) => value !== mvp[index]);
    }, start.mvp);
    const orbited = await state();
    await page.mouse.wheel(0, -240);
    await page.waitForFunction(mvp => {
        const now = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection'));
        return now.mvp.some((value, index) => value !== mvp[index]);
    }, orbited.mvp);
    assert.equal((await state()).objects, element.objects);
    assert((await state()).mvp.every(Number.isFinite));
    await command('Close', /^Close works while drawing a polyline or curve$/);
    assert.equal((await state()).objects, element.objects, 'Close must not clear the document');
    const beforeDrawing = await state();
    await command('Polyline', /^Polyline: First point/);
    await command('w2000,0,0', /^Polyline: Next point/);
    await command('w2300,0,0', /^Polyline: Next point/);
    await command('Close', /^Close needs at least three points$/);
    assert.deepEqual((await state()).drawing.points, [[2000, 0, 0], [2300, 0, 0]]);
    assert.equal((await state()).objects, beforeDrawing.objects);
    assert.equal((await state()).undo_depth, beforeDrawing.undo_depth);
    await command('w2300,400,0', /^Polyline: Next point/);
    assert.deepEqual((await state()).drawing.points, [[2000, 0, 0], [2300, 0, 0], [2300, 400, 0]]);
    await command('Close', /^Created and selected polyline /);
    await wait('drawing', null);
    await wait('objects', beforeDrawing.objects + 1);
    await wait('selected_kind', 'Polyline');
    const closed = await state();
    assert.equal(closed.undo_depth, beforeDrawing.undo_depth + 1, 'Close commits one original object');
    await command('Length', /^Length 1200 mm · 1 curve$/);
    assert.equal((await state()).mark.label, '1200 mm', 'The 500 mm closing edge completes the 300/400/500 triangle');
    await canvasFocus();
    await page.keyboard.press('ControlOrMeta+z');
    await wait('objects', beforeDrawing.objects);
    await page.keyboard.press('ControlOrMeta+Shift+z');
    await wait('objects', beforeDrawing.objects + 1);
    await command('Select By Name Polyline', /^1 selected: names containing/);
    await wait('selected_kind', 'Polyline');
    assert.deepEqual((await state()).identity, closed.identity, 'Redo restores the same closed original');
    assert.deepEqual((await state()).selected_geometry, closed.selected_geometry, 'Redo preserves all closed source points');
    await canvasFocus();
    await page.keyboard.press('ControlOrMeta+z');
    await wait('objects', beforeDrawing.objects);
    // Replace through the maintained loader; its clear path cancels streams and drops original owners.
    const empty = new URL('course-empty.yaml', page.url()).href;
    await page.route(empty, route => route.fulfill({status: 200, contentType: 'text/yaml', body: 'name: empty\nitems: []\n'}));
    await page.evaluate(url => {
        if (typeof window.wasmBindings?.reload_scene !== 'function') throw Error('Maintained reload_scene export is missing');
        window.wasmBindings.reload_scene(url);
    }, empty);
    await wait('objects', 0);
    await wait('cloud_points', 0);
    await wait('vertices', 0);
    await wait('source_cpu_known_payload_bytes', 0);
    assert.equal((await state()).pick_busy, false);
    assert.deepEqual(errors, []);
};
