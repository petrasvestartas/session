module.exports = async (page, helpers) => {
    await require('./adapter-proof.cjs')(page, helpers);
    await require('./errors-command.cjs')(page, helpers);
};
