module.exports = async (page, helpers) => {
    await require('./edit-owner.cjs')(page, helpers);
    await helpers.command(page, 'Orbit Right'); await helpers.command(page, 'Fit');
};
