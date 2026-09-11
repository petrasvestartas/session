"""Measure every label of an SVG in real Chrome: overflow and collisions, without playwright.

`check_illustrations.cjs` does this properly and pins each measured width, but it needs
playwright. This needs only an installed Chrome, so it runs where playwright does not:

    python3 docs/check_svg.py docs/illustrations/*.svg

It renders each file, reads every label's real bounding box and fails on a label that leaves
the canvas or genuinely overlaps another. Stacked lines 18px apart share descender space, so a
small vertical overlap is normal and only a real collision is reported.

It also reads the colour a reader actually sees: the label's own fill against whatever shape
lies under it. Dark ink on a shape that the dark-page remapping turned dark is invisible, and
nothing in the geometry says so, so the contrast ratio is checked as well.
"""
import json, re, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

PAGE = r"""<!doctype html><meta charset="utf-8"><body style="margin:0;background:#ffffff"><style>svg{display:block;max-width:none}</style>%s
<script>
const rgb = s => (s.match(/[\d.]+/g) || [255, 255, 255]).slice(0, 3).map(Number);
const over = (fg, a, bg) => fg.map((v, i) => v * a + bg[i] * (1 - a));
const lum = c => {
  const f = c.map(v => { v /= 255; return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4); });
  return 0.2126 * f[0] + 0.7152 * f[1] + 0.0722 * f[2];
};
// The ink box in rendered coordinates: getBBox is tight but local, getBoundingClientRect is
// rendered but includes the whole font box, and lines set 20px apart would read as colliding.
const ink = el => {
  const b = el.getBBox(), m = el.getScreenCTM();
  const pts = [[b.x, b.y], [b.x + b.width, b.y], [b.x, b.y + b.height], [b.x + b.width, b.y + b.height]]
    .map(([x, y]) => new DOMPoint(x, y).matrixTransform(m));
  const xs = pts.map(p => p.x), ys = pts.map(p => p.y);
  const x = Math.min(...xs), y = Math.min(...ys);
  return {x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y};
};
// Exact containment, not the bounding box: a label beside a triangle is inside that
// triangle's box and on the page's own ground, which read as the same colour and are not.
const inside = (el, x, y) => {
  const r = el.getBoundingClientRect();
  if (x < r.left || x > r.right || y < r.top || y > r.bottom) return false;
  const ctm = el.getScreenCTM();
  if (!ctm || !el.isPointInFill) return true;
  const p = new DOMPoint(x, y).matrixTransform(ctm.inverse());
  try { return el.isPointInFill(p); } catch (e) { return true; }
};
window.addEventListener('load', () => {
  const svg = document.querySelector('svg');
  // An SVG with only a viewBox stretches to the window, and every measurement with it: the
  // pinned textLength would then be the stretched width. Pin the drawing to its own size first.
  const vb = svg.viewBox.baseVal;
  svg.setAttribute('width', vb.width);
  svg.setAttribute('height', vb.height);
  // Rendered geometry, not getBBox: a generated diagram nests its content under a transform,
  // so local coordinates from two labels are not even in the same space, let alone the
  // canvas's. Everything below is in pixels relative to the drawing's own top-left corner.
  const box = svg.getBoundingClientRect();
  const page = rgb(getComputedStyle(document.body).backgroundColor);
  // Paint order is document order, so the ground under a label is the LAST filled shape that
  // both precedes it and encloses it. Geometry, not hit testing: a drawing taller than the
  // window has most of its labels outside the viewport, where elementsFromPoint sees nothing.
  const nodes = [...svg.querySelectorAll('rect,circle,ellipse,polygon,path,text')];
  const out = [];
  nodes.forEach((n, i) => {
    if (n.tagName !== 'text') return;
    const r = ink(n);
    const cx = r.x + r.width / 2, cy = r.y + r.height / 2;
    const cs = getComputedStyle(n);
    const fg = over(rgb(cs.fill), parseFloat(cs.fillOpacity || 1), page);
    let back = page;
    for (let j = 0; j < i; j++) {
      const el = nodes[j];
      if (el.tagName === 'text') continue;
      const es = getComputedStyle(el);
      const a = parseFloat(es.fillOpacity || 1);
      if (!es.fill || es.fill === 'none' || a === 0) continue;
      if (inside(el, cx, cy)) back = over(rgb(es.fill), a, back);
    }
    const l1 = lum(fg), l2 = lum(back);
    out.push({s: n.textContent.slice(0, 40), x: r.x - box.x, y: r.y - box.y, w: r.width, h: r.height,
              c: (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05)});
  });
  document.title = JSON.stringify({w: box.width, h: box.height, t: out});
});
</script></body>"""


# Below this a label is not merely low contrast, it is unreadable: 3:1 is the WCAG floor for
# large text, and every label here is 12.5px or more on a plain ground.
MIN_CONTRAST = 3.0


def measure(path):
    """Every label's real bounding box, in document order, from a real browser."""
    html = PAGE % Path(path).read_text()
    with tempfile.NamedTemporaryFile("w", suffix=".html", delete=False) as fh:
        fh.write(html); tmp = fh.name
    dom = subprocess.run(["google-chrome", "--headless", "--disable-gpu", "--no-sandbox",
                          "--virtual-time-budget=3000", "--dump-dom", f"file://{tmp}"],
                         capture_output=True, text=True, timeout=120).stdout
    m = re.search(r"<title>(.*?)</title>", dom, re.S)
    if not m:
        return None
    return json.loads(m.group(1).replace("&quot;", '"').replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">"))


def generated(path):
    """True for a drawing a tool owns, which must stay byte-identical to what that tool writes.

    Pinning one is not merely pointless - `diagrams.py --check` and `locator.py --check` compare
    bytes, so a pinned diagram reports itself out of date for good. Only the hand-placed text of
    draw.py needs a measured width written back.
    """
    name = Path(path).name
    return (name == "map.svg" or name.startswith(("locator-", "strip-"))
            or "data-d2-version" in Path(path).read_text(errors="ignore")[:400])


def pin(path):
    """Write each label's measured width back as textLength, the way the node checker does."""
    if generated(path):
        return []
    data = measure(path)
    if data is None:
        return [f"{path}: could not measure"]
    text = Path(path).read_text()
    parts, at = [], 0
    widths = [t["w"] for t in data["t"]]
    index = 0
    for m in re.finditer(r"<text\b([^>]*)>", text):
        if index >= len(widths):
            break
        attrs = m.group(1)
        if "textLength" not in attrs and widths[index] > 0:
            new = f"<text{attrs} textLength=\"{widths[index]:.1f}\" lengthAdjust=\"spacingAndGlyphs\">"
            parts.append(text[at:m.start()]); parts.append(new); at = m.end()
        index += 1
    parts.append(text[at:])
    Path(path).write_text("".join(parts))
    return []


def check(path):
    data = measure(path)
    if data is None:
        return [f"{path}: could not measure"]
    bad = []
    for t in data["t"]:
        if t["x"] < -1 or t["x"] + t["w"] > data["w"] + 1:
            bad.append(f"{Path(path).name}: overflows width ({t['x']:.0f}+{t['w']:.0f} > {data['w']:.0f}): {t['s']!r}")
        if t["y"] < -1 or t["y"] + t["h"] > data["h"] + 1:
            bad.append(f"{Path(path).name}: overflows height: {t['s']!r}")
        if t.get("c", 99) < MIN_CONTRAST:
            bad.append(f"{Path(path).name}: label contrast {t['c']:.1f}:1 against what is behind it: {t['s']!r}")
    boxes = data["t"]
    for i in range(len(boxes)):
        for j in range(i + 1, len(boxes)):
            a, b = boxes[i], boxes[j]
            ox = min(a["x"] + a["w"], b["x"] + b["w"]) - max(a["x"], b["x"])
            oy = min(a["y"] + a["h"], b["y"] + b["h"]) - max(a["y"], b["y"])
            if ox > 3 and oy > 7:  # stacked lines 18px apart share descender space; only real overlap counts
                bad.append(f"{Path(path).name}: labels overlap: {a['s']!r} / {b['s']!r}")
    return bad

def one(path, writing):
    return (pin(path) if writing else []) + check(path)



if __name__ == "__main__":
    args = sys.argv[1:]
    writing = "--write" in args
    files = [a for a in args if a != "--write"]
    # Two Chrome runs per file, a couple of seconds each: a whole directory is minutes serially.
    with ThreadPoolExecutor(max_workers=4) as pool:
        problems = [line for group in pool.map(lambda f: one(f, writing), files) for line in group]
    verb = "pinned and checked" if writing else "checked"
    if problems:
        print("\n".join(problems))
        sys.exit(1)
    print(f"PASS {len(files)} illustrations {verb}: no overflow, no collisions, readable contrast")
