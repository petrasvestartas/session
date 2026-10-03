module.exports = async (page, helpers) => {
    await require('./edit-reply.cjs')(page, helpers);
    await helpers.command(page, 'Move 0,0,0.15'); await helpers.command(page, 'Fit');
};
