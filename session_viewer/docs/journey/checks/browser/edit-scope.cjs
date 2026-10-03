module.exports = async (page, helpers) => {
    await require('./edit-intent.cjs')(page, helpers);
    await helpers.command(page, 'Move 0,0.25,0');
    await helpers.command(page, 'Fit');
};
