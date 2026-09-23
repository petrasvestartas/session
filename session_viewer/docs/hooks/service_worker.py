import hashlib
import json
import os
import posixpath
import re

ASSET = re.compile(r'(<link rel="(?:stylesheet|icon)" href="|<script src=")([^":?#]+)"')
WORKER = re.compile(r'"search": "([^":?#]+)"')
UNHASHED = {"sw.js", "sitemap.xml", "sitemap.xml.gz"}

serving = False
versions = {}
shell = set()


def on_startup(command, dirty):
    """Remember whether this run is `mkdocs serve`, which gets no service worker."""
    global serving
    serving = command == "serve"


def on_config(config):
    """Hash every extra stylesheet and script, so a page names the exact file it was built with."""
    versions.clear()
    shell.clear()
    for item in list(config.extra_css) + list(config.extra_javascript):
        with open(os.path.join(config.docs_dir, str(item)), "rb") as file:
            versions[str(item)] = hashlib.sha256(file.read()).hexdigest()[:8]


def on_post_page(output, page, config):
    """Version the extra stylesheets and scripts, and note every file a page loads as the worker's shell."""
    folder = page.url if page.url.endswith("/") else posixpath.dirname(page.url)

    def get_asset(asset):
        path = posixpath.normpath(posixpath.join(folder, asset.group(2))).lstrip("/")
        version = versions.get(path)
        if version is None:
            shell.add(path)
            return asset.group(0)
        shell.add("%s?v=%s" % (path, version))
        return '%s%s?v=%s"' % (asset.group(1), asset.group(2), version)

    worker = WORKER.search(output)
    if worker:
        shell.add(
            posixpath.normpath(posixpath.join(folder, worker.group(1))).lstrip("/")
        )
    return ASSET.sub(get_asset, output)


def on_post_build(config):
    """Write sw.js at the site root with this build's id and shell; a new build gets a new cache."""
    if serving:
        return
    digest = hashlib.sha256()
    for folder, names, files in os.walk(config.site_dir):
        names.sort()
        for name in sorted(files):
            path = os.path.join(folder, name)
            relative = os.path.relpath(path, config.site_dir)
            if relative in UNHASHED:
                continue
            digest.update(relative.encode())
            with open(path, "rb") as file:
                digest.update(file.read())
    with open(
        os.path.join(os.path.dirname(__file__), "sw.js"), encoding="utf-8"
    ) as file:
        source = file.read()
    source = source.replace("__BUILD__", digest.hexdigest()[:12]).replace(
        "__SHELL__", json.dumps(sorted(shell))
    )
    with open(os.path.join(config.site_dir, "sw.js"), "w", encoding="utf-8") as file:
        file.write(source)
