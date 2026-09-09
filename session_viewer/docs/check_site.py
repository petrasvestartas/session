#!/usr/bin/env python3
"""Check built lesson listings, exact downloads, explicit lexers and local document links."""
import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

HERE = Path(__file__).resolve().parent


class Page(HTMLParser):
    """Read browser-visible code, anchor IDs and link targets without a browser dependency."""

    def __init__(self, path):
        super().__init__(convert_charrefs=True)
        self.links, self.ids, self.languages, self.code = [], set(), set(), []
        self.in_pre = False
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
        if tag == "pre":
            self.in_pre = True
            self.code.append("")

    def handle_endtag(self, tag):
        if tag == "pre":
            self.in_pre = False

    def handle_data(self, data):
        if self.in_pre:
            self.code[-1] += data


def check(site):
    """Compare rendered complete files with the independent checkpoint hash inventory."""
    series = json.loads((HERE / "reconstruction/series.json").read_text())
    pages = {path.resolve(): Page(path) for path in site.rglob("*.html") if path.is_file()}
    errors, checked, languages = [], 0, set()
    for step in series["steps"]:
        directory = site / "lessons" / step["id"]
        index = directory / "index.html"
        if index.resolve() not in pages:
            errors.append(f'missing lesson file index {step["id"]}')
            continue
        patch = (HERE / "reconstruction" / step["patch"]).read_text()
        # Empty added files have a Git header but no +++ hunk header.
        changed = []
        deleted = []
        for section in re.split(r"^diff --git ", patch, flags=re.MULTILINE)[1:]:
            name = section.splitlines()[0].split(" b/", 1)[1]
            if "deleted file mode " in section:
                deleted.append(name)
            else:
                changed.append(name)
        expected_downloads = {name.replace("/", "--") + ".txt" for name in changed}
        actual_downloads = {path.name for path in (directory / "raw").glob("*.txt")}
        if actual_downloads != expected_downloads:
            errors.append(f'incomplete download set at {step["id"]}: '
                          f'{sorted(actual_downloads ^ expected_downloads)}')
        for name in deleted:
            if name not in index.read_text():
                errors.append(f'unlisted removal at {step["id"]}: {name}')
        for raw in (directory / "raw").glob("*.txt"):
            name = raw.name.removesuffix(".txt").replace("--", "/")
            expected = step["files"].get(name)
            if expected != hashlib.sha256(raw.read_bytes()).hexdigest():
                errors.append(f'exact download differs: {step["id"]}/{name}')
            listing = directory / raw.name.removesuffix(".txt") / "index.html"
            page = pages.get(listing.resolve())
            if page is None:
                errors.append(f'missing complete file page: {listing}')
                continue
            # HTML code blocks normalize the terminal line break. Every other character,
            # including internal blank lines, must still match the exact downloadable file.
            source = raw.read_text().rstrip("\n")
            if not any(code.rstrip("\n") == source for code in page.code):
                errors.append(f'rendered code differs: {step["id"]}/{name}')
            suffix = Path(name).suffix
            required = {".rs": "rust", ".wgsl": "wgsl", ".toml": "toml"}.get(suffix)
            if required and required not in page.languages:
                errors.append(f'missing {required} highlighting: {listing}')
            languages.update(page.languages)
            checked += 1
    for path, page in pages.items():
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
    if errors:
        raise ValueError("\n".join(errors))
    print(f"PASS {len(series['steps'])} lesson indexes, {checked} rendered/downloaded complete files, "
          f"{len(pages)} HTML pages, local links and explicit Rust/WGSL/TOML highlighting")


def main():
    """Check the default maintained build or an explicitly supplied built site."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--site", type=Path, default=HERE.parent / "target/docs/site")
    options = parser.parse_args()
    check(options.site.resolve())


if __name__ == "__main__":
    main()
