const path = require('node:path');
module.exports = async (page, {command, step}, tail) => {
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
    await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    for (const line of ['Select Next', 'Move 0.35,0,0.25', 'Move 0.25,0,0.15', 'Move 0,-0.5,0', 'View Isometric',
        'Orbit Right', 'Move 0.1,0,0', 'Orbit Up', 'Orbit Right', 'Orbit Up', 'Orbit Right', 'Move 0.05,0,0',
        'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0',
        'Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0', ...tail, 'Fit']) await command(page, line);
};
