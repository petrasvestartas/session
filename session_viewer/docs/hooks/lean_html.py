import hashlib
import io
import itertools
import os
import posixpath
import re
import struct
from urllib.parse import unquote

try:
    from PIL import Image
except ImportError:
    Image = None

HIGHLIGHT = re.compile(r'<div class="[^"]*\bhighlight\b[^"]*">.*?</div>', re.S)
# Material leaves .w unstyled and paints .n in the plain code colour;
# stop unwrapping .n if --md-code-hl-name-color is ever themed.
# Only a token between tags or line breaks is unwrapped: text merged into
# a neighbouring run would move the glyphs after it by 1/64 px.
BARE = r'(?<=[>\n])<span class="%s">([^<]*)</span>(?=[<\n])'
SPACE = re.compile(BARE % "w")
NAME = re.compile(BARE % "n")
PRE = re.compile(r"<pre\b")
IMAGE = re.compile(r"<img\b[^>]*>")
SOURCE = re.compile(r'\ssrc="([^"]+)"')
# site path of a PNG's lossless WebP twin -> its bytes, written after the build
twins = {}


def get_code(block):
    """A code block without the spans the theme leaves unstyled, kept out of the search index."""
    code = NAME.sub(r"\1", SPACE.sub(r"\1", block.group(0)))
    return PRE.sub('<pre data-search-exclude="1"', code, 1)


def get_size(path):
    """Pixel size of a PNG, None for any other image."""
    with open(path, "rb") as file:
        head = file.read(24)
    if not head.startswith(b"\x89PNG\r\n\x1a\n"):
        return None
    return struct.unpack(">II", head[16:24])


def get_webp(path, config):
    """Lossless WebP of a PNG, the same pixels in fewer bytes; None when not smaller or Pillow is missing."""
    if Image is None:
        return None
    with open(path, "rb") as file:
        png = file.read()
    cache = os.path.join(
        os.path.dirname(config.config_file_path),
        "target",
        "docs",
        "webp",
        hashlib.sha256(png).hexdigest() + ".webp",
    )
    if os.path.exists(cache):
        with open(cache, "rb") as file:
            webp = file.read()
    else:
        buffer = io.BytesIO()
        Image.open(io.BytesIO(png)).save(
            buffer, "WEBP", lossless=True, quality=90, method=5, exact=True
        )
        webp = buffer.getvalue()
        os.makedirs(os.path.dirname(cache), exist_ok=True)
        # renamed into place: a stopped or parallel build never leaves or reads a short file
        partial = "%s.%d" % (cache, os.getpid())
        with open(partial, "wb") as file:
            file.write(webp)
        os.replace(partial, cache)
    return webp if len(webp) < len(png) else None


def get_image(tag, page, files, lazy, config):
    """A local PNG's img tag with its size, when lazy deferred loading, and a WebP twin when smaller."""
    source = SOURCE.search(tag)
    if (
        source is None
        or ":" in source.group(1)
        or source.group(1).startswith("/")
        or " width=" in tag
    ):
        return tag
    path = posixpath.normpath(
        posixpath.join(posixpath.dirname(page.url), unquote(source.group(1)))
    )
    file = files.get_file_from_path(path)
    if file is None:
        return tag
    size = get_size(file.abs_src_path)
    if size is None:
        return tag
    extra = ' width="%d" height="%d"' % size
    if lazy:
        extra += ' loading="lazy" decoding="async"'
    image = tag[:4] + extra + tag[4:]
    webp = get_webp(file.abs_src_path, config)
    if webp is None:
        return image
    twins[posixpath.splitext(file.dest_uri)[0] + ".webp"] = webp
    return '<picture><source type="image/webp" srcset="%s.webp">%s</picture>' % (
        posixpath.splitext(source.group(1))[0],
        image,
    )


def on_page_content(html, page, config, files):
    """Unwrap unstyled code tokens, keep code out of search, size local PNGs, lazy-load all but a first image with no code above it, and give PNGs WebP twins."""
    html = HIGHLIGHT.sub(get_code, html)
    code = html.find("<pre")
    order = itertools.count()
    return IMAGE.sub(
        lambda image: get_image(
            image.group(0),
            page,
            files,
            next(order) > 0 or 0 <= code < image.start(),
            config,
        ),
        html,
    )


def on_post_build(config):
    """Write the WebP twins next to their PNGs."""
    for path, webp in twins.items():
        with open(os.path.join(config.site_dir, path), "wb") as file:
            file.write(webp)
    twins.clear()
