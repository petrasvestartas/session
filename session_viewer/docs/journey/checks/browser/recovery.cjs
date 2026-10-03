module.exports = async (page, helpers) => {
    await require('./replace.cjs')(page, helpers);
    await require('./startup.cjs')(page, helpers);
    await helpers.command(page, 'Orbit Up');
    await helpers.command(page, 'Fit');
};
