module.exports = async (page, helpers) => {
    await require('./reload-reply.cjs')(page, helpers);
    await helpers.command(page, 'Move 0,0.15,0'); await helpers.command(page, 'Fit');
};
