#!/usr/bin/env python3
"""Test the API index without MCP"""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent
LANGUAGES = ['python', 'cpp', 'rust']

SOURCE_FILES = {
    'python': [REPO_ROOT / "session_py/src/session_py/point.py", REPO_ROOT / "session_py/src/session_py/color.py"],
    'cpp': [REPO_ROOT / "session_cpp/src/point.h", REPO_ROOT / "session_cpp/src/point.cpp", REPO_ROOT / "session_cpp/src/color.h", REPO_ROOT / "session_cpp/src/color.cpp"],
    'rust': [REPO_ROOT / "session_rust/src/point.rs", REPO_ROOT / "session_rust/src/color.rs"],
}

def parse_python(content):
    methods = {}
    current_class = None
    for match in re.finditer(r'^class\s+(\w+)|^(\s*)def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:', content, re.MULTILINE):
        if match.group(1): current_class = match.group(1)
        elif match.group(3):
            indent, name, params, ret = match.group(2, 3, 4, 5)
            if current_class and indent:
                key = f"{current_class}.{name}"
                methods[key] = {'signature': f"{name}({params.replace('self, ', '').replace('self', '')})" + (f" -> {ret}" if ret else "")}
    return methods

def parse_cpp(content):
    methods = {}
    for match in re.finditer(r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)', content, re.MULTILINE):
        ret, cls, name, params = match.groups()
        methods[f"{cls}.{name}"] = {'signature': f"{ret.strip()} {name}({params})"}
    return methods

def parse_rust(content):
    methods = {}
    current_impl = None
    for match in re.finditer(r'^impl(?:<[^>]+>)?\s+(\w+)|^\s*pub\s+fn\s+(\w+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)(?:\s*->\s*([^\{]+))?', content, re.MULTILINE):
        if match.group(1): current_impl = match.group(1)
        elif match.group(2) and current_impl:
            name, params, ret = match.group(2, 3, 4)
            clean_params = params.replace('&self, ', '').replace('&self', '').replace('&mut self, ', '').replace('&mut self', '')
            methods[f"{current_impl}.{name}"] = {'signature': f"{name}({clean_params})" + (f" -> {ret.strip()}" if ret else "")}
    return methods

PARSERS = {'python': parse_python, 'cpp': parse_cpp, 'rust': parse_rust}

class APIIndex:
    def __init__(self):
        self.methods = {}
        for lang, files in SOURCE_FILES.items():
            for file in files:
                if file.exists():
                    for key, data in PARSERS[lang](file.read_text(errors='ignore')).items():
                        if key not in self.methods: self.methods[key] = {}
                        self.methods[key][lang] = {**data, 'file': file.name}
    
    def get_method(self, name, langs=None):
        for key, impls in self.methods.items():
            if key == name or key.endswith(f".{name}"):
                if langs: impls = {k: v for k, v in impls.items() if k in langs}
                return {'name': key, 'implementations': impls} if impls else None
        return None
    
    def search(self, query):
        return [{'name': k, 'languages': list(v.keys())} for k, v in self.methods.items() if query.lower() in k.lower()][:10]
    
    def list_class(self, cls):
        return sorted([k for k in self.methods if k.startswith(f"{cls}.")])
    
    def list_classes(self):
        return sorted(set(k.split('.')[0] for k in self.methods if '.' in k))

index = APIIndex()

# Test
print(f"Indexed {len(index.methods)} methods")
print(f"Classes: {index.list_classes()}")
print()

# Test queries
tests = [
    ("Point.distance", None),
    ("distance", None),
    ("new", "rust"),
    ("__init__", "python"),
]

for method, lang in tests:
    print(f"--- get_api('{method}', {lang}) ---")
    result = index.get_method(method, [lang] if lang else None)
    if result:
        print(f"Found: {result['name']}")
        for l, impl in result['implementations'].items():
            print(f"  {l}: {impl['signature']}")
    else:
        print("Not found")
    print()

print("--- search_api('distance') ---")
for r in index.search('distance'):
    print(f"  {r['name']} ({', '.join(r['languages'])})")

print("\n--- list_methods('Point') ---")
print(index.list_class('Point'))
