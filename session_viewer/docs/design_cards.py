"""Build the Design System cards this course publishes back to claude.ai/design.

theme.py pulls: the design system's tokens become theme.css. This pushes the other way, so the
drawing vocabulary the course actually uses is visible next to the system that themes it - the
BRG Equilibrium palette, the six box kinds, the plate every figure is mounted on, and all
twenty-one illustrations as one set. Nothing here is authored by hand: every colour comes from
draw.py and every chrome value from theme.json, so a card can never drift from the site.
"""
from pathlib import Path
import importlib.util
import json
import shutil

HERE = Path(__file__).resolve().parent
OUT = HERE.parent / "target/docs/cards"
GROUP = "Session Viewer"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, HERE / path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


DRAW = load("draw", "illustrations/draw.py")
TOKENS = load("theme", "stylesheets/theme.py").compute_tokens(json.loads((HERE / "stylesheets/theme.json").read_text()))
THEME = json.loads((HERE / "stylesheets/theme.json").read_text())

MEANING = {
    "navy": "compression", "pink": "tension", "green": "loads", "yellow": "hover",
    "yellow_light": "selection fill", "orange": "warning", "ghost": "ghosted ink",
    "grey": "plain stroke", "zero": "zero / arrow", "pink_band": "GPU, WGSL",
    "blue_band": "CPU, Rust", "zero_band": "note", "black": "diagram ground",
    "text2": "secondary label", "page": "page", "white": "box fill",
}
KIND_MEANING = {
    "cpu": "a CPU or Rust box", "gpu": "a GPU or WGSL box", "note": "an aside",
    "sel": "the selected thing", "warn": "the trap being described", "plain": "anything else",
}

STYLE = """
body { background: var(--color-bg); color: var(--color-text); margin: 0; }
.card { max-width: 860px; margin: 0 auto; padding: 32px 24px 40px; }
.head { font-size: 11px; letter-spacing: .1em; text-transform: uppercase; opacity: .55; margin: 28px 0 12px; }
.head:first-child { margin-top: 0; }
.note { font-size: 12px; opacity: .6; max-width: 62ch; line-height: 1.6; }
code { font-family: ui-monospace, "SF Mono", Menlo, monospace; font-size: .92em; }
.swatches { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.sw { height: 40px; border-radius: RADIUS; border: 1px solid color-mix(in srgb, var(--color-text) 18%, transparent); }
.swl { font-size: 10px; opacity: .55; margin-top: 5px; }
.swh { font-size: 10px; opacity: .35; font-family: ui-monospace, Menlo, monospace; }
.kinds { display: grid; grid-template-columns: repeat(3, 1fr); gap: 14px; background: BLACK; border-radius: RADIUS; padding: 18px; }
.kind { border-radius: RADIUS; padding: 10px 12px; font: 13px system-ui, sans-serif; color: #111111; }
.kindl { font-size: 10px; opacity: .55; margin-top: 6px; color: #c9ccd6; font-family: system-ui, sans-serif; }
.plate { background: var(--color-surface); border: 1px solid HAIRLINE; border-radius: RADIUS; padding: 12px; }
.plate img { display: block; width: 100%; }
.figures { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
.figname { font-size: 10px; opacity: .45; margin-top: 6px; font-family: ui-monospace, Menlo, monospace; }
"""


def style():
    return (STYLE.replace("RADIUS", TOKENS["--ds-radius"])
                 .replace("HAIRLINE", TOKENS["--ds-hairline"])
                 .replace("BLACK", DRAW.PAL["black"]))


def page(name, subtitle, viewport, body):
    return "\n".join([
        f'<!-- @dsCard group="{GROUP}" name="{name}" subtitle="{subtitle}" viewport="{viewport}" -->',
        "<!doctype html>",
        '<html lang="en">',
        "<head>",
        '<meta charset="utf-8" />',
        '<meta name="viewport" content="width=device-width, initial-scale=1" />',
        f"<title>{name}</title>",
        '<link rel="stylesheet" href="../styles.css" />',
        f"<style>{style()}</style>",
        "</head>",
        "<body>",
        f'<main class="card">{body}</main>',
        "</body>",
        "</html>",
        "",
    ])


def palette_card():
    swatches = "".join(
        f'<div><div class="sw" style="background: {value}"></div>'
        f'<div class="swl">{MEANING.get(key, key)}</div><div class="swh">{value}</div></div>'
        for key, value in DRAW.PAL.items()
    )
    kinds = "".join(
        f'<div><div class="kind" style="background: {fill}; border: 1.5px solid {stroke}">{key}</div>'
        f'<div class="kindl">{KIND_MEANING[key]}</div></div>'
        for key, (fill, stroke) in DRAW.KIND.items()
    )
    body = "".join([
        '<div class="head">Drawing palette</div>',
        f'<div class="swatches">{swatches}</div>',
        '<div class="head">Box kinds</div>',
        f'<div class="kinds">{kinds}</div>',
        '<div class="head">Why it is not the theme</div>',
        '<p class="note">These are the BRG Equilibrium colours, and they are the drawing’s own. '
        'A reader carries them from a diagram to the running viewer, where the same navy, pink and '
        'yellow mark compression, tension and selection, so the design system themes the page around '
        'them rather than replacing them. Edit them in <code>docs/illustrations/draw.py</code>.</p>',
    ])
    return page("Drawing palette", "The illustrations' own colours and the six box kinds", "860x620", body)


def plate_card():
    figure = "spaces.svg"
    body = "".join([
        '<div class="head">Mounted</div>',
        f'<div class="plate"><img src="figures/{figure}" alt="A figure on its plate" /></div>',
        '<div class="head">The rule</div>',
        f'<p class="note">Every figure on the site is mounted, never floated: one mat in the system’s '
        f'surface, one hairline, one <code>{THEME["radius"]}px</code> radius, the same for a generated SVG, '
        f'a screenshot and a black Mermaid diagram. That is what makes twenty-one illustrations and '
        f'twenty-eight screenshots read as one set. The system’s <code>imageTreatment: '
        f'{THEME["imageTreatment"]}</code> is where the decision comes from; <code>docs/stylesheets/course.css</code> '
        f'is where it is spent.</p>',
    ])
    return page("Figure plate", f'imageTreatment "{THEME["imageTreatment"]}" - how every figure is mounted', "860x620", body)


def figures_card(names):
    tiles = "".join(
        f'<div><div class="plate"><img src="figures/{name}" alt="{name}" /></div>'
        f'<div class="figname">{name}</div></div>'
        for name in names
    )
    body = "".join([
        '<div class="head">Course figures</div>',
        f'<div class="figures">{tiles}</div>',
        '<div class="head">Generated, then measured</div>',
        '<p class="note">Each one is emitted by <code>docs/illustrations/draw.py</code>, which sizes every '
        'box from its longest line, and then measured by <code>docs/check_illustrations.cjs</code> in real '
        'Chrome, which fails on any label that leaves its box and pins each measured width so another '
        'machine’s fonts cannot overflow either. Redraw rather than retouch.</p>',
    ])
    return page("Course figures", f"{len(names)} generated illustrations, one set", "860x2400", body)


def main():
    figures = sorted(path.name for path in (HERE / "illustrations").glob("*.svg"))
    if OUT.exists():
        shutil.rmtree(OUT)
    (OUT / "session_viewer/figures").mkdir(parents=True)
    for name in figures:
        shutil.copy(HERE / "illustrations" / name, OUT / "session_viewer/figures" / name)
    (OUT / "session_viewer/palette.html").write_text(palette_card())
    (OUT / "session_viewer/plate.html").write_text(plate_card())
    (OUT / "session_viewer/figures.html").write_text(figures_card(figures))


if __name__ == "__main__":
    main()

# description: build the Design System cards for the "Classical" project at claude.ai/design.
# directory: cd ~/code/code_cpp/wood_research/session/session_viewer
# run: python3 docs/design_cards.py
# push: the bundle lands in target/docs/cards/; upload session_viewer/** with the DesignSync tool.
