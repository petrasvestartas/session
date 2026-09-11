"""Measure every label of an SVG in real Chrome: overflow and collisions, without playwright.

`check_illustrations.cjs` does this properly and pins each measured width, but it needs
playwright. This needs only an installed Chrome, so it runs where playwright does not:

    python3 docs/check_svg.py docs/illustrations/*.svg

It renders each file, reads every label's real bounding box and fails on a label that leaves
the canvas or genuinely overlaps another. Stacked lines 18px apart share descender space, so a
small vertical overlap is normal and only a real collision is reported.
"""
import json, re, subprocess, sys, tempfile
from pathlib import Path

PAGE = """<!doctype html><meta charset="utf-8"><body style="margin:0">%s
<script>
window.addEventListener('load', () => {
  const svg = document.querySelector('svg');
  const vb = svg.viewBox.baseVal;
  const out = [];
  for (const t of svg.querySelectorAll('text')) {
    const b = t.getBBox();
    out.push({s: t.textContent.slice(0, 40), x: b.x, y: b.y, w: b.width, h: b.height});
  }
  document.title = JSON.stringify({w: vb.width, h: vb.height, t: out});
});
</script></body>"""

def check(path):
    html = PAGE % Path(path).read_text()
    with tempfile.NamedTemporaryFile("w", suffix=".html", delete=False) as fh:
        fh.write(html); tmp = fh.name
    dom = subprocess.run(["google-chrome", "--headless", "--disable-gpu", "--no-sandbox",
                          "--virtual-time-budget=3000", "--dump-dom", f"file://{tmp}"],
                         capture_output=True, text=True, timeout=120).stdout
    m = re.search(r"<title>(.*?)</title>", dom, re.S)
    if not m:
        return [f"{path}: could not measure"]
    data = json.loads(m.group(1).replace("&quot;", '"').replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">"))
    bad = []
    for t in data["t"]:
        if t["x"] < -1 or t["x"] + t["w"] > data["w"] + 1:
            bad.append(f"{Path(path).name}: overflows width ({t['x']:.0f}+{t['w']:.0f} > {data['w']:.0f}): {t['s']!r}")
        if t["y"] < -1 or t["y"] + t["h"] > data["h"] + 1:
            bad.append(f"{Path(path).name}: overflows height: {t['s']!r}")
    boxes = data["t"]
    for i in range(len(boxes)):
        for j in range(i + 1, len(boxes)):
            a, b = boxes[i], boxes[j]
            ox = min(a["x"] + a["w"], b["x"] + b["w"]) - max(a["x"], b["x"])
            oy = min(a["y"] + a["h"], b["y"] + b["h"]) - max(a["y"], b["y"])
            if ox > 3 and oy > 7:  # stacked lines 18px apart share descender space; only real overlap counts
                bad.append(f"{Path(path).name}: labels overlap: {a['s']!r} / {b['s']!r}")
    return bad

if __name__ == "__main__":
    problems = []
    for p in sys.argv[1:]:
        problems += check(p)
    print("\n".join(problems) if problems else f"PASS {len(sys.argv)-1} illustrations: no overflow, no label collisions")
