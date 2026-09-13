"""Maintain a full-viewer result at the end of each implementation tutorial."""
import argparse
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = json.loads((HERE / "tutorial-results.json").read_text())
HEADING = "## Expected viewer result"


def render(page):
    result = RESULTS[page]
    title = page.removesuffix(".md").replace("-", " ")
    return (f'{HEADING}\n\n{result["caption"]}\n\n'
            f'[![Full viewer result for {title}](screenshots/{result["image"]})]'
            f'(screenshots/{result["image"]})\n')


def check():
    tutorials = {path.name for pattern in ("[0-2][0-9]*.md", "current-*.md", "*-tutorial.md")
                 for path in HERE.glob(pattern)}
    missing = tutorials - RESULTS.keys()
    if missing:
        raise ValueError(f'tutorials without result screenshots: {sorted(missing)}')
    for name, result in RESULTS.items():
        if not (HERE / "screenshots" / result["image"]).is_file():
            raise ValueError(f'{name}: missing result screenshot {result["image"]}')
        if not (HERE / name).read_text().endswith(render(name)):
            raise ValueError(f'{name}: must end with its expected viewer result')
    print(f"PASS {len(RESULTS)} tutorials end with a full-viewer result")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true")
    if parser.parse_args().write:
        for name in RESULTS:
            page = HERE / name
            body = page.read_text().split("\n" + HEADING + "\n", 1)[0]
            page.write_text(body.rstrip() + "\n\n" + render(name))
    check()


if __name__ == "__main__":
    main()
