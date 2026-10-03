module.exports = async (page, helpers) => {
    await require('./reload-complete.cjs')(page, helpers);
    await helpers.command(page, 'Move 0,0,0.1'); await helpers.command(page, 'Fit');
};
