const assert = require('node:assert/strict');
async function ui(page) {
  return page.evaluate(() => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-ui') || 'null'));
}
async function button(page, prefix, label) {
  await page.waitForFunction(({prefix,label}) => {
    const raw = document.querySelector('canvas')?.getAttribute('data-viewer-ui');
    return raw && JSON.parse(raw).controls.some(control => control.key.startsWith(prefix) && control.label.includes(label));
  }, {prefix,label});
  const control = (await ui(page)).controls.filter(control => control.key.startsWith(prefix) && control.label.includes(label)).at(-1);
  const [x0,y0,x1,y1] = control.rect;
  await page.mouse.move((x0+x1)/2,(y0+y1)/2);
  await page.waitForTimeout(60);
  await page.mouse.click((x0+x1)/2,(y0+y1)/2);
  await page.waitForTimeout(120);
}
async function typeCommand(page, text) {
  await page.locator('canvas').focus();
  if (!(await ui(page)).command_open) {
    await page.keyboard.press(':');
    await page.waitForTimeout(250);
  }
  await button(page, 'command/input', 'Command');
  await page.keyboard.press('Control+a');
  await page.keyboard.type(text, {delay: 8});
  await page.waitForTimeout(80);
  assert.equal((await ui(page)).command,text);
}
async function command(page, text, close = true) {
  await typeCommand(page,text);
  await page.keyboard.press('Enter');
  await page.waitForFunction(text => {
    const raw = document.querySelector('canvas')?.getAttribute('data-viewer-ui');
    return raw && JSON.parse(raw).history.at(-1)?.startsWith(`> ${text}\n`);
  },text);
  if(close) await button(page,'command/close','Close');
}
module.exports={ui,button,typeCommand,command};
