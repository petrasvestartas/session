#!/usr/bin/env python3
"""Check the built course site: local links and anchors, downloads, explicit lexers, no test docs."""
import argparse
from html.parser import HTMLParser
import hashlib
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

HERE = Path(__file__).resolve().parent


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
    print(f"PASS {len(pages)} pages, {downloads} exact downloads, local links and Rust/WGSL/TOML lexers")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--site", type=Path, default=HERE.parent / "target/docs/site")
    check(parser.parse_args().site.resolve())


if __name__ == "__main__":
    main()
