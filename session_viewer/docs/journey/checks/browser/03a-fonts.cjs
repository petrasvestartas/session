const assert = require('node:assert/strict');
module.exports = async (page, {step}) => {
    await page.addInitScript(() => {
        if (typeof GPUDevice === 'undefined') return;
        window.__fontOwner = {shaders: [], pipelines: []};
        for (const [method, field] of [['createShaderModule', 'shaders'], ['createRenderPipeline', 'pipelines'], ['createRenderPipelineAsync', 'pipelines']]) {
            const original = GPUDevice.prototype[method];
            if (!original) continue;
            GPUDevice.prototype[method] = function (descriptor, ...rest) {
                window.__fontOwner[field].push(descriptor.label || '');
                return original.call(this, descriptor, ...rest);
            };
        }
    });
    await page.reload();
    await page.waitForFunction(status => document.querySelector('#status')?.textContent === status, step.browser_status);
    const owner = await page.evaluate(() => window.__fontOwner);
    assert(owner.shaders.includes('egui'), 'The introductory browser must construct the actual egui shader owner');
    assert(owner.pipelines.includes('egui_pipeline'), 'The GPU painter must create its pipeline before drawing is connected');
    console.log(`${step.id}: actual egui shader and pipeline constructed; interface drawing remains unconnected`);
};
