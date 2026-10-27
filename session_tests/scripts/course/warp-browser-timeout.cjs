const requested = process.env.VIEWER_WARP_ACTION_TIMEOUT_MS;

if (requested !== undefined) {
    if (requested !== '120000') throw Error('VIEWER_WARP_ACTION_TIMEOUT_MS must equal 120000');
    const {chromium} = require('playwright');
    const launch = chromium.launch;
    chromium.launch = async function (...args) {
        const browser = await launch.apply(this, args);
        const newContext = browser.newContext;
        browser.newContext = async function (...options) {
            const context = await newContext.apply(this, options);
            context.setDefaultTimeout(120000);
            return context;
        };
        return browser;
    };
    console.log('WARP context timeout: 120000ms; explicit operation timeouts unchanged.');
}
