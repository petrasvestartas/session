// Fetch the scene manifest and its first file while the wasm downloads. app/fetch.rs takes
// each request by URL; the URLs follow app/route.rs and app/live.rs, and a mismatch is unused.
(function () {
  var BUCKET = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
  var LOCAL_URL = /^http:\/\/(localhost|127\.0\.0\.1|\[::1\]):/;
  var early = window.__viewerEarly = {};
  var query = {};

  location.search.slice(1).split("&").forEach(function (pair) {
    var at = pair.indexOf("=");
    var key = at < 0 ? pair : pair.slice(0, at);
    if (!(key in query))
      try {
        query[key] = at < 0 ? "" : decodeURIComponent(pair.slice(at + 1));
      } catch (error) {
        query[key] = null;
      }
  });

  function join(base, file) {
    return /^https?:\/\//.test(file) ? file : base + file.replace(/^(\.\/)+/, "");
  }

  function immutable(url) {
    var hash = url.split("/").pop().split(".")[0].split("-").pop();
    return hash.length >= 16 && /^[0-9a-f]+$/i.test(hash);
  }

  function start(key, url, range) {
    var controller = new AbortController();
    var init = { mode: "cors", cache: immutable(url) ? "default" : "no-cache", signal: controller.signal };
    if (range)
      init.headers = { Range: "bytes=0-8191" };
    var reply = fetch(url, init);
    reply.catch(function () {});
    early[key] = { r: reply, c: controller };
    return reply;
  }

  var local = /^(localhost|127\.0\.0\.1|\[::1\]|::1)$/.test(location.hostname);
  var scene = query.scene;
  if (scene !== undefined && (!scene || scene[0] === "/" || /\/\/|:/.test(scene) || scene.split("/").indexOf("..") >= 0))
    scene = undefined;
  var last = location.pathname.split("/").pop();
  if (scene === undefined && last && !/\.html$|:|^\./.test(last))
    scene = last;
  var data = query.data;
  var base = data === undefined ? BUCKET : data === "off" || data === "" ? "" :
    /^https:\/\//.test(data) || LOCAL_URL.test(data) ? data : BUCKET;
  if (base && base.slice(-1) !== "/")
    base += "/";
  var live = query.live;
  var manifest = null;
  var whole = false;

  if (live !== "off" && live !== "0" && (live !== undefined || scene === "view_live" || (!scene && !local))) {
    manifest = live && (/^https:\/\//.test(live) || LOCAL_URL.test(live)) ? live : BUCKET + "scenes/view_live.yaml";
    whole = true;
    if (!base || manifest.indexOf(base) !== 0)
      base = manifest.slice(0, manifest.lastIndexOf("/") + 1);
  } else if (scene) {
    var path = scene.indexOf(".") >= 0 ? scene : scene + ".yaml";
    manifest = join(base, path.indexOf("/") >= 0 ? path : "scenes/" + path);
  } else if (local) {
    manifest = "view_local.yaml";
    base = "";
  }

  if (!manifest)
    return;

  start(manifest, manifest, false).then(function (reply) {
    return reply.ok ? reply.clone().text() : "";
  }).then(function (text) {
    var hit = /^[\s-]*\{?\s*file\s*[:=]\s*["']?([^"'\s#,}]+)/m.exec(text) || /"file"\s*:\s*"([^"]+)"/.exec(text);
    if (!hit)
      return;
    var file = join(base, hit[1]);
    // the item's own lines; a range read of a stored-gzip file corrupts the cache entry
    var rest = text.slice(hit.index + hit[0].length);
    var end = rest.search(/\n[ \t]*-|\n[^\s#]/);
    var item = end < 0 ? rest : rest.slice(0, end);
    if (whole)
      start(file, file, false);
    else if (/\.pb$/.test(file) && !/["']?encoding["']?\s*[:=]/.test(item))
      start(file + "#probe", file, true);
  }).catch(function () {});
})();
