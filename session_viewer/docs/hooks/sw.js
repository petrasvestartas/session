// Written to the site root by service_worker.py with this build's id and the files every page loads.
const BUILD = "__BUILD__";
const SHELL = __SHELL__;
const SCOPE = new URL("./", self.location).pathname;
// one github.io origin serves every project page, so a cache name carries the scope
const PREFIX = "course " + SCOPE + " ";
const CACHE = PREFIX + BUILD;
// a Material asset with a content hash in its name, or an extra stylesheet or script with ?v=<hash>
const FIXED = /\.[0-9a-f]{8}\.min\.(css|js)$|\?v=[0-9a-f]{8}$/;

const page = (url) => {
  const at = new URL(url);
  return at.origin + at.pathname;
};
const stamp = (response) => response.headers.get("etag") + " " + response.headers.get("last-modified");

self.addEventListener("install", (event) => {
  // a shell file that fails to load is fetched when a page asks for it, it never holds back the update
  event.waitUntil(caches.open(CACHE).then((cache) => Promise.all(SHELL.map((file) => cache.add(file).catch(() => {})))).then(() => self.skipWaiting()));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(activate());
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== "GET" || url.origin !== self.location.origin || !url.pathname.startsWith(SCOPE) || url.pathname === self.location.pathname || request.headers.has("range"))
    return;
  if (request.mode === "navigate" || url.pathname.endsWith("/search/search_index.json"))
    event.respondWith(stale(event, request));
  else
    event.respondWith(first(event, request, FIXED.test(url.pathname + url.search)));
});

async function activate() {
  // the build before this one stays for pages still open on it; older builds go
  const older = (await caches.keys()).filter((key) => key.startsWith(PREFIX) && key !== CACHE);
  for (const key of older.slice(0, -1))
    await caches.delete(key);
  await self.clients.claim();
  // pages already open were fetched before this worker existed
  const cache = await caches.open(CACHE);
  for (const client of await self.clients.matchAll({type: "window"}))
    if (new URL(client.url).pathname.startsWith(SCOPE))
      await cache.add(new Request(page(client.url), {cache: "no-cache"})).catch(() => {});
}

// only into this build's cache, and never again once retire() deleted it
function keep(key, response) {
  if (!response.ok || response.redirected || response.type !== "basic")
    return;
  const copy = response.clone();
  return caches.has(CACHE).then((kept) => kept && caches.open(CACHE).then((cache) => cache.put(key, copy)));
}

// a file whose URL names its content comes from any build's cache; any other file from this build's
async function first(event, request, fixed) {
  const hit = await caches.match(request, fixed ? {} : {cacheName: CACHE});
  if (hit)
    return hit;
  // the HTTP cache may still hold the build before for max-age, and this build's cache would keep it
  const response = await fetch(request, fixed ? {} : {cache: "no-cache"});
  event.waitUntil(Promise.resolve(keep(request, response)).catch(() => {}));
  return response;
}

// a page or the search index answers from the cache at once and asks the server (not the HTTP cache) for the next visit
async function stale(event, request) {
  const key = request.mode === "navigate" ? page(request.url) : request.url;
  const hit = await caches.match(key, {cacheName: CACHE});
  if (!hit || hit.redirected) {
    const response = await fetch(request);
    event.waitUntil(Promise.resolve(keep(key, response)).catch(() => {}));
    return response;
  }
  event.waitUntil(revalidate(key, hit).catch(() => {}));
  return hit;
}

// a changed file means a deploy: it joins this build's cache only while the server still ships this build's worker
async function revalidate(key, hit) {
  const response = await fetch(key, {cache: "no-cache"});
  if (stamp(response) !== stamp(hit)) {
    const worker = await fetch(self.location.href, {cache: "no-cache"});
    if (worker.status === 404)
      return retire();
    if (!(await worker.text()).includes('"' + BUILD + '"'))
      return;
  }
  return keep(key, response);
}

// the site no longer ships a worker: unregister and take this scope's caches along
async function retire() {
  await self.registration.unregister();
  for (const key of await caches.keys())
    if (key.startsWith(PREFIX))
      await caches.delete(key);
}
