module.exports = async (page, helpers) => {
    for (let i = 0; i < 5; i++) await helpers.command(page, 'Zoom Out');
};
