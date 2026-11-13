export function repositorySourceLink(relative: string): string {
  const [first, ...rest] = relative.split('/');
  const submodule = /^session_(cpp|py|rust|data|proto)$/.test(first);
  const repository = submodule ? first : 'session';
  const file = submodule ? rest.join('/') : relative;
  return `https://github.com/petrasvestartas/${repository}/blob/main/${file}`;
}

export function registerLinkedAsset(assets: Map<string, string>, key: string, file: string, label?: string): string {
  assets.set(key, file);
  const source = label !== undefined && (/\.rs$/i.test(key) || (/\.html?$/i.test(key) && /\bsource\b/i.test(label)));
  if (!source) return key;
  const preview = key + '.txt';
  assets.set(preview, file);
  return preview;
}
