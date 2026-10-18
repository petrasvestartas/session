module.exports = async (page, helpers) => {
    await require('./sub-pick.cjs')(page, helpers);
    await require('./multi-selection-view.cjs')(page, helpers);
};
