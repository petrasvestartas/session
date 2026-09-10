#!/usr/bin/env python3
"""Check the built course site: theme, local links and anchors, downloads, explicit lexers, no test docs."""
import argparse
from html.parser import HTMLParser
import hashlib
import importlib.util
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

HERE = Path(__file__).resolve().parent
THEME = importlib.util.spec_from_file_location("theme", HERE / "stylesheets/theme.py")


def check_theme(errors):
    """theme.css must still be what theme.py renders from the exported design system."""
    module = importlib.util.module_from_spec(THEME)
    THEME.loader.exec_module(module)
    theme = json.loads((HERE / "stylesheets/theme.json").read_text())
    if module.render(theme) != (HERE / "stylesheets/theme.css").read_text():
        errors.append("theme.css is stale: run python3 docs/stylesheets/theme.py")
    faces = json.loads((HERE / "stylesheets/fonts/fonts.json").read_text())["faces"]
    families = {face["family"] for face in faces}
    css = (HERE / "stylesheets/course.css").read_text()
    for family in families:
        if f'font-family: "{family}"' not in css:
            errors.append(f'"{family}" is committed but course.css does not declare it')
    if "Roboto" not in families:
        errors.append("the course is set in Roboto: it must stay committed under stylesheets/fonts")
    for face in faces:
        path = HERE / "stylesheets/fonts" / face["file"]
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != face["sha256"]:
            errors.append(f'{face["file"]} is missing or differs from fonts.json')


class Page(HTMLParser):
    """Read anchor IDs, link targets and code languages without a browser."""

    def __init__(self, path):
        super().__init__(convert_charrefs=True)
        self.links, self.ids, self.languages = [], set(), set()
        self.feed(path.read_text())

    def handle_starttag(self, tag, attributes):
        values = dict(attributes)
        if "id" in values:
            self.ids.add(values["id"])
        for name in ("href", "src"):
            if name in values:
                self.links.append(values[name])
        for value in values.get("class", "").split():
            if value.startswith("language-"):
                self.languages.add(value.removeprefix("language-"))


def check(site):
    series = json.loads((HERE / "reconstruction/series.json").read_text())
    pages = {path.resolve(): Page(path) for path in site.rglob("*.html") if path.is_file()}
    errors, languages, downloads = [], set(), 0
    check_theme(errors)
    for step in series["steps"]:
        raw = site / "lessons" / step["id"] / "raw"
        for path in raw.glob("*.txt"):
            name = path.name.removesuffix(".txt").replace("--", "/")
            expected = step["files"].get(name)
            if expected != hashlib.sha256(path.read_bytes()).hexdigest():
                errors.append(f'download differs from checkpoint {step["id"]}: {name}')
            downloads += 1
    for path, page in pages.items():
        languages.update(page.languages)
        for link in page.links:
            url = urlsplit(link)
            if url.scheme or url.netloc or link.startswith("/"):
                continue
            target = (path.parent / unquote(url.path)).resolve() if url.path else path
            if target.is_dir():
                target = target / "index.html"
            if not target.exists():
                errors.append(f'broken link in {path.relative_to(site)}: {link}')
            elif url.fragment and target in pages and unquote(url.fragment) not in pages[target].ids:
                errors.append(f'broken anchor in {path.relative_to(site)}: {link}')
    if not {"rust", "wgsl", "toml"} <= languages:
        errors.append("the built course must include Rust, WGSL and TOML lexers")
    for path in pages:
        if "/tests/" in path.as_posix():
            errors.append(f"test documentation is published: {path.relative_to(site)}")
    if errors:
        raise ValueError("\n".join(errors))
    theme = json.loads((HERE / "stylesheets/theme.json").read_text())["name"]
    print(f'PASS {len(pages)} pages, {downloads} exact downloads, local links, Rust/WGSL/TOML lexers, "{theme}" theme')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--site", type=Path, default=HERE.parent / "target/docs/site")
    check(parser.parse_args().site.resolve())


if __name__ == "__main__":
    main()
