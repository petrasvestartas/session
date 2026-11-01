import argparse
import hashlib
import json
import re
import xml.etree.ElementTree as ET
from collections import defaultdict
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


def digest(data):
    return hashlib.sha256(data).hexdigest()


def svg_digest(text):
    root = ET.fromstring(text)
    ids = {node.attrib['id']: f'id{index}' for index, node in enumerate(
        node for node in root.iter() if 'id' in node.attrib)}

    def attribute(key, value):
        if key == 'id':
            return ids[value]
        value = re.sub(r'url\(\s*([\"\']?)#([^\s)\"\']+)\1\s*\)',
                       lambda match: 'url(#' + ids.get(match[2], match[2]) + ')', value)
        if key.split('}')[-1] == 'href' and value.startswith('#'):
            value = '#' + ids.get(value[1:], value[1:])
        if key in ['aria-labelledby', 'aria-describedby']:
            value = ' '.join(ids.get(item, item) for item in value.split())
        return value

    def node(item, parent_text=False):
        text = parent_text or item.tag.split('}')[-1] in ['text', 'tspan', 'textPath']
        content, tail = item.text or '', item.tail or ''
        if not text and not content.strip():
            content = ''
        if not parent_text and not tail.strip():
            tail = ''
        return [item.tag, sorted((key, attribute(key, value)) for key, value in item.attrib.items()),
                content, [node(child, text) for child in item], tail]

    return digest(json.dumps(node(root), ensure_ascii=False, separators=(',', ':')).encode())


def without_fences(text):
    result, fence = [], None
    for line in text.splitlines(keepends=True):
        marker = re.match(r'^\s*(`{3,}|~{3,})([^\r\n]*)', line)
        if fence:
            if marker and marker[1][0] == fence[0] and len(marker[1]) >= len(fence) and not marker[2].strip():
                fence = None
        elif marker:
            fence = marker[1]
        else:
            result.append(line)
    if fence:
        raise ValueError('Unclosed Markdown code fence')
    return re.sub(r'<!--.*?-->', '', ''.join(result), flags=re.S)


class Images(HTMLParser):
    def __init__(self):
        super().__init__()
        self.urls = []

    def handle_starttag(self, tag, attributes):
        for key, value in attributes:
            if value and (tag == 'img' and key == 'src' or
                          tag == 'image' and key in ['href', 'xlink:href'] or
                          tag in ['object', 'embed'] and key in ['data', 'src']):
                self.urls.append(value)


def audit(docs):
    docs = Path(docs).resolve()
    book = json.loads((docs / 'journey/book.json').read_bytes())
    uses, errors, pages = [], [], []
    for chapter, section in enumerate(book['chapters'], 1):
        if not section.get('consolidated'):
            errors.append({'chapter': chapter, 'error': 'Primary consolidated pages are required'})
        for lesson in section.get('consolidated', []):
            page = docs / lesson['page']
            owner = {'chapter': chapter, 'chapterTitle': section['title'], 'page': lesson['page']}
            try:
                raw = page.read_bytes()
                text = without_fences(raw.decode('utf-8'))
                pages.append({**owner, 'sha256': digest(raw)})
                svgs = list(re.finditer(r'<svg\b.*?</svg\s*>', text, re.S | re.I))
                if len(svgs) != len(re.findall(r'<svg\b', text, re.I)):
                    raise ValueError('Nested or unclosed inline SVG')
                for index, match in enumerate(svgs, 1):
                    uses.append({**owner, 'kind': 'svg', 'asset': f'inline-svg-{index}',
                                 'sha256': svg_digest(match[0])})
                html = Images()
                html.feed(text)
                urls = html.urls + [match[1] for match in re.finditer(
                    r'!\[[^\]\n]*\]\(\s*(<[^>\n]+>|[^\s)]+)(?:\s+[^)]*)?\)', text)]
                definitions = dict(re.findall(r'^\s*\[([^\]]+)\]:\s*<?([^\s>]+)>?', text, re.M))
                for match in re.finditer(r'!\[([^\]\n]*)\]\[([^\]\n]*)\]', text):
                    label = match[2] or match[1]
                    if label not in definitions:
                        raise ValueError('Missing image reference definition: ' + label)
                    urls.append(definitions[label])
                for url in dict.fromkeys(urls):
                    parsed = urlsplit(url.strip('<>'))
                    if parsed.scheme or parsed.netloc:
                        continue
                    name = unquote(parsed.path)
                    suffix = Path(name).suffix.lower()
                    if suffix not in ['.png', '.jpg', '.jpeg', '.webp', '.svg']:
                        continue
                    asset = (page.parent / name).resolve()
                    data = asset.read_bytes()
                    uses.append({**owner, 'kind': 'svg' if suffix == '.svg' else 'raster',
                                 'asset': str(asset.relative_to(docs)), 'physicalSha256': digest(data),
                                 'sha256': svg_digest(data.decode('utf-8')) if suffix == '.svg' else digest(data)})
            except (OSError, ValueError, ET.ParseError) as error:
                errors.append({**owner, 'error': str(error)})
    groups = defaultdict(list)
    for use in uses:
        groups[(use['kind'], use['sha256'])].append(use)
    shared = [{'kind': kind, 'sha256': sha, 'chapters': sorted({use['chapter'] for use in owners}),
               'owners': owners} for (kind, sha), owners in groups.items()
              if len({use['chapter'] for use in owners}) > 1]
    return {'status': 'FAIL' if errors or shared else 'PASS', 'chapters': len(book['chapters']),
            'pages': pages, 'illustrationOccurrences': len(uses), 'illustrations': uses,
            'crossChapterSharedIllustrations': shared, 'errors': errors,
            'scope': 'Primary consolidated reader pages only. Raster bytes and ID-normalized SVG structure; no perceptual or browser-render claim. Within-chapter reuse is permitted. No allowlist.'}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Check primary reader illustration ownership across chapters.')
    parser.add_argument('docs', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    result = audit(args.docs)
    text = json.dumps(result, indent=2) + '\n'
    if args.output:
        with args.output.open('x', encoding='utf-8', newline='\n') as output:
            output.write(text)
        print(json.dumps({'status': result['status'], 'pages': len(result['pages']),
                          'crossChapterGroups': len(result['crossChapterSharedIllustrations']),
                          'errors': len(result['errors']), 'output': str(args.output)}))
    else:
        print(text, end='')
    raise SystemExit(result['status'] != 'PASS')
