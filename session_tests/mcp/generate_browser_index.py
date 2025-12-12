#!/usr/bin/env python3
"""Generate browser-compatible API index for Vue CLI"""

import re
import json
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent

SOURCE_FILES = {
    'python': [REPO_ROOT / "session_py/src/session_py/point.py", REPO_ROOT / "session_py/src/session_py/color.py", REPO_ROOT / "session_py/src/session_py/vector.py", REPO_ROOT / "session_py/src/session_py/tolerance.py", REPO_ROOT / "session_py/src/session_py/line.py"],
    'cpp': [REPO_ROOT / "session_cpp/src/point.h", REPO_ROOT / "session_cpp/src/point.cpp", REPO_ROOT / "session_cpp/src/color.h", REPO_ROOT / "session_cpp/src/color.cpp", REPO_ROOT / "session_cpp/src/vector.h", REPO_ROOT / "session_cpp/src/vector.cpp", REPO_ROOT / "session_cpp/src/tolerance.h", REPO_ROOT / "session_cpp/src/tolerance.cpp", REPO_ROOT / "session_cpp/src/line.h", REPO_ROOT / "session_cpp/src/line.cpp"],
    'rust': [REPO_ROOT / "session_rust/src/point.rs", REPO_ROOT / "session_rust/src/color.rs", REPO_ROOT / "session_rust/src/vector.rs", REPO_ROOT / "session_rust/src/tolerance.rs", REPO_ROOT / "session_rust/src/line.rs"],

}

def parse_python(content):
    methods, cls = {}, None
    for m in re.finditer(r'^class\s+(\w+)|^(\s*)def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:', content, re.MULTILINE):
        if m.group(1): cls = m.group(1)
        elif m.group(3) and cls and m.group(2):
            start = m.end()
            code = m.group(0) + '\n' + '\n'.join(content[start:start+800].split('\n')[:12])
            methods[f"{cls}.{m.group(3)}"] = {'sig': f"{m.group(3)}({m.group(4).replace('self, ', '').replace('self', '')})", 'code': code.strip()}
    return methods

def parse_cpp(content):
    methods = {}
    for m in re.finditer(r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)', content, re.MULTILINE):
        start, end = m.start(), content.find('}', m.end())
        code = content[start:end+1] if end > 0 else content[start:start+400]
        methods[f"{m.group(2)}.{m.group(3)}"] = {'sig': f"{m.group(1).strip()} {m.group(3)}({m.group(4)})", 'code': code[:500].strip()}
    return methods

def parse_rust(content):
    methods, impl = {}, None
    for m in re.finditer(r'^impl(?:<[^>]+>)?\s+(\w+)|^\s*pub\s+fn\s+(\w+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)(?:\s*->\s*([^\{]+))?', content, re.MULTILINE):
        if m.group(1): impl = m.group(1)
        elif m.group(2) and impl:
            start = m.start()
            depth, i = 0, content.find('{', m.end())
            if i > 0:
                depth = 1
                i += 1
                while i < len(content) and depth > 0:
                    if content[i] == '{': depth += 1
                    elif content[i] == '}': depth -= 1
                    i += 1
            code = content[start:i][:500]
            params = m.group(3).replace('&self, ', '').replace('&self', '').replace('&mut self, ', '').replace('&mut self', '')
            methods[f"{impl}.{m.group(2)}"] = {'sig': f"{m.group(2)}({params})" + (f" -> {m.group(4).strip()}" if m.group(4) else ""), 'code': code.strip()}
    return methods

# Build unified concept index
concepts = {}
parsers = {'python': parse_python, 'cpp': parse_cpp, 'rust': parse_rust}

for lang, files in SOURCE_FILES.items():
    for f in files:
        if f.exists():
            for key, data in parsers[lang](f.read_text(errors='ignore')).items():
                if key not in concepts:
                    concepts[key] = {'name': key, 'implementations': {}}
                concepts[key]['implementations'][lang] = data

# Convert to list format for browser
output = {
    'version': '3.0',
    'type': 'concept-unified',
    'concepts': list(concepts.values())
}

# Write to public folder
out_path = Path(__file__).parent.parent / "public" / "apiIndex.js"
out_path.parent.mkdir(exist_ok=True)

with open(out_path, 'w') as f:
    f.write("// Auto-generated unified API index\n")
    f.write("window.API_INDEX = ")
    json.dump(output, f, indent=2)
    f.write(";\n")

print(f"Generated {out_path}")
print(f"  {len(concepts)} concepts indexed")
print(f"  Size: {out_path.stat().st_size / 1024:.1f} KB")
