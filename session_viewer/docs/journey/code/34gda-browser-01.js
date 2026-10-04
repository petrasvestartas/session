function text(value) {
    if (typeof value !== 'string') return '';
    let result = '', count = 0;
    for (const letter of value) { if (count++ === 256) break; result += letter; }
    return result;
}

export function begin() {
    const gpu = navigator.gpu, original = gpu.requestAdapter;
    const previous = Object.getOwnPropertyDescriptor(gpu, 'requestAdapter');
    let active = true, value = null;
    async function wrapped(...args) {
        const adapter = await Reflect.apply(original, this, args);
        if (active && adapter) {
            try {
                const info = adapter.info;
                value = JSON.stringify({vendor: text(info.vendor), architecture: text(info.architecture),
                    device: text(info.device), description: text(info.description)});
            } catch { value = null; }
        }
        return adapter;
    }
    Object.defineProperty(gpu, 'requestAdapter', {value: wrapped, writable: true, configurable: true});
    return {read: () => value, stop() {
        active = false; value = null;
        if (Object.getOwnPropertyDescriptor(gpu, 'requestAdapter')?.value !== wrapped) return;
        if (previous) Object.defineProperty(gpu, 'requestAdapter', previous);
        else delete gpu.requestAdapter;
    }};
}
