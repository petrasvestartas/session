module.exports = async (page, helpers) => {
    await require('./reload-bridge.cjs')(page, helpers);
    await helpers.command(page, 'Move 0.1,0,0'); await helpers.command(page, 'Fit');
};
