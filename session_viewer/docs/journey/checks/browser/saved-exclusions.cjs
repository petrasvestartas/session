module.exports = async (page, helpers) => {
    await require('./saved-recency.cjs')(page, helpers);
    await helpers.command(page, 'Move 0,-0.05,0'); await helpers.command(page, 'Fit');
};
