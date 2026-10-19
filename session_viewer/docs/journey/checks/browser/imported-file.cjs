const fs = require('node:fs');

module.exports = async (page, file) => {
    const bytes = fs.readFileSync(file), magic = Buffer.from('SESSION-VIEWER\x01\n');
    const status = bytes.subarray(0, magic.length).equals(magic)
        ? 'Document replaced. Undo restores the previous document.'
        : 'File imported. Undo removes the entire import.';
    await page.waitForFunction(status => document.getElementById('status')?.textContent === status, status);
};
