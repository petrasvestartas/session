module.exports = async canvas => 'data:image/png;base64,' + (await canvas.screenshot()).toString('base64');
