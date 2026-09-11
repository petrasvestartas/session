/* A pinned copy of the viewer map: the step you are reading stays on screen while you scroll.
   Each step's inline map carries the file name of a compact strip in data-strip. The step that
   counts as current is the last one whose heading is above the middle of the viewport - the same
   rule a table of contents uses, so the bar and the highlighted heading always agree. */
(function () {
  function headingFor(img) {
    var node = img.closest("p") || img;
    while (node) {
      if (/^H[1-6]$/.test(node.tagName) ) return node;
      node = node.previousElementSibling;
    }
    return null;
  }

  function start() {
    var maps = Array.prototype.slice.call(document.querySelectorAll("img.locator[data-strip]"));
    if (!maps.length) return;
    // mkdocs rewrites the markdown image path but not our attributes, so borrow the directory
    // the first inline map resolved to.
    var base = (maps[0].getAttribute("src") || "").replace(/[^/]+$/, "");

    // Anchors, finest first: one per code block, falling back to the step heading.
    var anchors = [];
    maps.forEach(function (img) {
      var head = headingFor(img);
      if (head) anchors.push({ at: head, strip: img.getAttribute("data-strip"),
                               where: (img.getAttribute("alt") || "")
                                 .replace(/^Where this step sits in the viewer: /, "")
                                 .replace(/,? with .*$/, "") });
    });
    Array.prototype.forEach.call(document.querySelectorAll(".zone-mark[data-strip]"), function (m) {
      anchors.push({ at: m, strip: m.getAttribute("data-strip"), where: m.getAttribute("data-zone") });
    });
    anchors.sort(function (a, b) {
      var d = a.at.compareDocumentPosition(b.at);
      return (d & Node.DOCUMENT_POSITION_FOLLOWING) ? -1 : 1;
    });
    var steps = anchors;
    if (!steps.length) return;

    var bar = document.createElement("div");
    bar.className = "viewer-strip";
    var img = document.createElement("img");
    img.alt = "Where you are in the viewer";
    var note = document.createElement("span");
    note.className = "viewer-strip-note";
    bar.appendChild(img);
    bar.appendChild(note);
    var article = document.querySelector(".md-content__inner") || document.body;
    article.insertBefore(bar, article.firstChild);

    var current = null;
    function place() {
      // Current is whatever last passed under the pinned bar: the code you are looking at, not
      // the one coming. The floor keeps an anchored heading (which lands below the bar) current.
      var rect = bar.getBoundingClientRect();
      var line = Math.max(rect.height ? rect.bottom + 24 : 0, 160);
      var pick = steps[0];
      for (var i = 0; i < steps.length; i++) {
        if (steps[i].at.getBoundingClientRect().top <= line) pick = steps[i];
      }
      if (pick === current) return;
      current = pick;
      img.src = base + (pick.strip || "").split("/").pop();
      note.textContent = pick.where ? "you are in: " + pick.where : "";
    }
    place();
    addEventListener("scroll", place, { passive: true });
    addEventListener("resize", place, { passive: true });
    addEventListener("hashchange", function () { setTimeout(place, 0); });
  }
  if (document.readyState === "loading") addEventListener("DOMContentLoaded", start);
  else start();
})();
