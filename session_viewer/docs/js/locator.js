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
    var steps = [];
    maps.forEach(function (img) {
      var head = headingFor(img);
      if (head) steps.push({ head: head, img: img });
    });
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
      var middle = innerHeight / 2;
      var pick = steps[0];
      for (var i = 0; i < steps.length; i++) {
        if (steps[i].head.getBoundingClientRect().top <= middle) pick = steps[i];
      }
      if (pick === current) return;
      current = pick;
      // mkdocs rewrites the markdown image path but not our attribute, so take the directory
      // the inline map resolved to and swap in the strip's file name.
      var here = pick.img.getAttribute("src") || "";
      var file = (pick.img.getAttribute("data-strip") || "").split("/").pop();
      img.src = here.replace(/[^/]+$/, file);
      var alt = pick.img.getAttribute("alt") || "";
      var where = alt.replace(/^Where this step sits in the viewer: /, "").replace(/,? with .*$/, "");
      note.textContent = where ? "you are in: " + where : "";
    }
    place();
    addEventListener("scroll", place, { passive: true });
    addEventListener("resize", place, { passive: true });
    addEventListener("hashchange", function () { setTimeout(place, 0); });
  }
  if (document.readyState === "loading") addEventListener("DOMContentLoaded", start);
  else start();
})();
