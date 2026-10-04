module.exports = async (page, helpers) => {
    await require('./phases-proof.cjs')(page, helpers);
    await require('./phases-adapter.cjs')(page, helpers);
    await require('./phases-errors.cjs')(page, helpers);
};
