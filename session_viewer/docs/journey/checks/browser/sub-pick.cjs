module.exports = async (page, helpers) => {
    await require('./stale-pick.cjs')(page, {...helpers, pickUniformBytes: 160, sourceOutlines: true});
    await require('./sub-pick-view.cjs')(page, helpers);
};
