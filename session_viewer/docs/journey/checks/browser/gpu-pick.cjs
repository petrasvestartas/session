module.exports = async (page, helpers) => {
    await require('./grid-input.cjs')(page, helpers);
    await require('./gpu-pick-view.cjs')(page, helpers);
};
