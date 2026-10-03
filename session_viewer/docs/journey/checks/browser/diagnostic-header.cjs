module.exports = async (page, helpers) => {
    await require('./device-stop.cjs')(page, helpers);
    await helpers.command(page, 'Orbit Up'); await helpers.command(page, 'Fit');
};
