module.exports = async (page, helpers) => {
    await require('./diagnostic-header.cjs')(page, helpers);
    await helpers.command(page, 'Orbit Right'); await helpers.command(page, 'Fit');
};
