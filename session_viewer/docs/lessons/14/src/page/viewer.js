// These named registrations belong to this document's lifetime and are installed once.
function preventContextMenu(event) {
  event.preventDefault();
}

function installCanvasHandlers() {
  var canvas = document.getElementById("canvas");
  if (canvas)
    canvas.addEventListener("contextmenu", preventContextMenu);
}

document.addEventListener("DOMContentLoaded", installCanvasHandlers, { once: true });

// Reload the scene in place when the embedding page asks for it, e.g. after
// an example was re-run and rewrote its .pb. Reloading this whole frame
// would restart WebGPU and reset the camera; this swaps only the geometry.
function reloadSceneMessage(event) {
  if (event.source !== window.parent && event.origin !== window.location.origin)
    return;
  var data = event.data;

  if (!data || data.type !== "session-viewer:reload-scene")
    return;

  if (window.wasmBindings && window.wasmBindings.reload_scene) {
    window.wasmBindings.reload_scene(data.scene || undefined);
  }
}

window.addEventListener("message", reloadSceneMessage);

if (!navigator.gpu) {
  document.body.insertAdjacentHTML("beforeend",
    '<div id="no-webgpu">WebGPU is unavailable. Open this viewer in a WebGPU-enabled browser over HTTPS or localhost; check the browser’s GPU settings.</div>');
}
