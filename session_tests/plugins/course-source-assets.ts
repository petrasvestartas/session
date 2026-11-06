export function registerLinkedAsset(assets: Map<string, string>, key: string, file: string, label?: string): string {
  assets.set(key, file);
  const source = label !== undefined && (/\.rs$/i.test(key) || (/\.html?$/i.test(key) && /\bsource\b/i.test(label)));
  if (!source) return key;
  const preview = key + '.txt';
  assets.set(preview, file);
  return preview;
}
