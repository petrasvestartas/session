#!/usr/bin/env python3
"""Simple CLI to test Session API without MCP"""

import re
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent

SOURCE_FILES = {
    'python': [REPO_ROOT / "session_py/src/session_py/point.py", REPO_ROOT / "session_py/src/session_py/color.py"],
    'cpp': [REPO_ROOT / "session_cpp/src/point.h", REPO_ROOT / "session_cpp/src/point.cpp", REPO_ROOT / "session_cpp/src/color.h", REPO_ROOT / "session_cpp/src/color.cpp"],
    'rust': [REPO_ROOT / "session_rust/src/point.rs", REPO_ROOT / "session_rust/src/color.rs"],
}

def parse_python(content):
    methods, cls = {}, None
    for m in re.finditer(r'^class\s+(\w+)|^(\s*)def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:', content, re.MULTILINE):
        if m.group(1): cls = m.group(1)
        elif m.group(3) and cls and m.group(2):
            start = m.end()
            code = m.group(0) + '\n' + '\n'.join(content[start:start+1000].split('\n')[:15])
            methods[f"{cls}.{m.group(3)}"] = {'sig': f"{m.group(3)}({m.group(4).replace('self, ', '').replace('self', '')})", 'code': code}
    return methods

def parse_cpp(content):
    methods = {}
    for m in re.finditer(r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)', content, re.MULTILINE):
        start, end = m.start(), content.find('}', m.end())
        code = content[start:end+1] if end > 0 else content[start:start+500]
        methods[f"{m.group(2)}.{m.group(3)}"] = {'sig': f"{m.group(1).strip()} {m.group(3)}({m.group(4)})", 'code': code[:600]}
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
            code = content[start:i][:600]
            params = m.group(3).replace('&self, ', '').replace('&self', '').replace('&mut self, ', '').replace('&mut self', '')
            methods[f"{impl}.{m.group(2)}"] = {'sig': f"{m.group(2)}({params})" + (f" -> {m.group(4).strip()}" if m.group(4) else ""), 'code': code}
    return methods

# Build index
index = {}
parsers = {'python': parse_python, 'cpp': parse_cpp, 'rust': parse_rust}
for lang, files in SOURCE_FILES.items():
    for f in files:
        if f.exists():
            for k, v in parsers[lang](f.read_text(errors='ignore')).items():
                if k not in index: index[k] = {}
                index[k][lang] = v

print(f"Session API - {len(index)} methods indexed\n")
print("Commands: get <method> | search <query> | list <class> | quit\n")

while True:
    try:
        cmd = input(">>> ").strip().split(maxsplit=1)
    except (EOFError, KeyboardInterrupt):
        break
    
    if not cmd: continue
    action = cmd[0].lower()
    arg = cmd[1] if len(cmd) > 1 else ""
    
    if action in ('q', 'quit', 'exit'):
        break
    
    elif action == 'get':
        found = None
        for k, v in index.items():
            if k == arg or k.endswith(f".{arg}"):
                found = (k, v)
                break
        if found:
            print(f"\n# {found[0]}\n")
            for lang in ['python', 'cpp', 'rust']:
                if lang in found[1]:
                    print(f"## {lang.upper()}: {found[1][lang]['sig']}")
                    print(f"```{lang}\n{found[1][lang]['code']}\n```\n")
        else:
            print(f"Not found: {arg}")
    
    elif action == 'search':
        results = [(k, list(v.keys())) for k, v in index.items() if arg.lower() in k.lower()][:10]
        for name, langs in results:
            print(f"  {name} ({', '.join(langs)})")
    
    elif action == 'list':
        methods = sorted([k.split('.')[-1] for k in index if k.startswith(f"{arg}.")])
        print(f"{arg}: {', '.join(methods)}")
    
    elif action == 'classes':
        print(sorted(set(k.split('.')[0] for k in index if '.' in k)))
    
    else:
        print("Commands: get <method> | search <query> | list <class> | classes | quit")
