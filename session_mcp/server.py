#!/usr/bin/env python3
"""
Session API MCP Server

Exposes Session geometry library API to Claude Code via MCP.
Query methods across Python, C++, and Rust implementations.

Usage:
    python -m session_mcp.server
    # or via entry point:
    session-mcp
"""

import re
import sys
from pathlib import Path
from mcp.server.fastmcp import FastMCP

# =============================================================================
# Configuration
# =============================================================================

REPO_ROOT = Path(__file__).parent.parent

CLASSES = [
    'color',
    'line',
    'mesh',
    'nurbscurve',
    'plane',
    'point',
    'pointcloud',
    'polyline',
    'tolerance',
    'vector',
    'xform',
]

SOURCE_FILES = {
    'python': [REPO_ROOT / f"session_py/src/session_py/{cls}.py" for cls in CLASSES],
    'cpp': [f for cls in CLASSES for f in [REPO_ROOT / f"session_cpp/src/{cls}.h", REPO_ROOT / f"session_cpp/src/{cls}.cpp"]],
    'rust': [REPO_ROOT / f"session_rust/src/{cls}.rs" for cls in CLASSES],
}

# =============================================================================
# Parsers - Extract method signatures and code from source files
# =============================================================================

def parse_python(content: str) -> dict:
    """Extract class methods from Python source."""
    methods = {}
    current_class = None
    for match in re.finditer(
        r'^class\s+(\w+)|^(\s*)def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:',
        content, re.MULTILINE
    ):
        if match.group(1):
            current_class = match.group(1)
        elif match.group(3):
            indent, name, params, ret = match.group(2, 3, 4, 5)
            if current_class and indent:
                key = f"{current_class}.{name}"
                start = match.end()
                lines = content[start:start+1500].split('\n')[:20]
                code = match.group(0) + '\n' + '\n'.join(lines)
                methods[key] = {
                    'signature': f"{name}({params.replace('self, ', '').replace('self', '')})" + (f" -> {ret}" if ret else ""),
                    'code': code.strip(),
                }
    return methods


def parse_cpp(content: str) -> dict:
    """Extract class methods from C++ source."""
    methods = {}
    constructor_scores = {}  # Track best constructor per class

    # Find class name from header
    class_match = re.search(r'class\s+(\w+)\s*[:{]', content)
    header_class = class_match.group(1) if class_match else None

    # C++ constructors in .cpp files: ClassName::ClassName(params)
    for match in re.finditer(
        r'^(\w+)::(\1)\s*\(([^)]*)\)\s*(?::[^{]*)?{',
        content, re.MULTILINE
    ):
        cls, name, params = match.groups()
        key = f"{cls}.constructor"
        start = match.start()
        end = content.find('}', match.end())
        code = content[start:end+1] if end > 0 else content[start:start+500]
        is_copy = cls in params and ('const' in params or '&' in params)
        has_primitives = 'double' in params or 'float' in params or 'int' in params
        score = 0 if is_copy else (2 if has_primitives else 1)
        if key not in methods or score > constructor_scores.get(cls, -1):
            constructor_scores[cls] = score
            methods[key] = {
                'signature': f"{cls}({params})",
                'code': code[:800].strip(),
            }

    # Header-style constructors: ClassName(params) : init {} or ClassName(params);
    if header_class:
        for match in re.finditer(
            rf'^\s+{header_class}\s*\(([^)]*)\)\s*(?::[^{{;]*)?(?:{{[^}}]*}}|;)',
            content, re.MULTILINE
        ):
            params = match.group(1)
            key = f"{header_class}.constructor"
            code = match.group(0).strip()
            is_copy = header_class in params and ('const' in params or '&' in params)
            has_primitives = 'double' in params or 'float' in params or 'int' in params
            score = 0 if is_copy else (2 if has_primitives else 1)
            if key not in methods or score > constructor_scores.get(header_class, -1):
                constructor_scores[header_class] = score
                methods[key] = {
                    'signature': f"{header_class}({params})",
                    'code': code[:800],
                }
    # Regular methods: ReturnType ClassName::method(params)
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)',
        content, re.MULTILINE
    ):
        ret, cls, name, params = match.groups()
        if cls == name:  # Skip constructors (already handled)
            continue
        key = f"{cls}.{name}"
        start = match.start()
        end = content.find('}', match.end())
        code = content[start:end+1] if end > 0 else content[start:start+500]
        methods[key] = {
            'signature': f"{ret.strip()} {name}({params})",
            'code': code[:800].strip(),
        }
    # Header declarations
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)\s*\(([^)]*)\)\s*(?:const)?\s*;',
        content, re.MULTILINE
    ):
        ret, name, params = match.groups()
        class_match = re.search(r'class\s+(\w+)\s*[:{]', content[:match.start()])
        if class_match:
            cls = class_match.group(1)
            key = f"{cls}.{name}"
            if key not in methods:
                methods[key] = {
                    'signature': f"{ret.strip()} {name}({params})",
                    'code': match.group(0).strip(),
                }
    return methods


def parse_rust(content: str) -> dict:
    """Extract impl methods from Rust source."""
    methods = {}
    current_impl = None
    for match in re.finditer(
        r'^impl(?:<[^>]+>)?\s+(\w+)|^\s*pub\s+fn\s+(\w+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)(?:\s*->\s*([^\{]+))?',
        content, re.MULTILINE
    ):
        if match.group(1):
            current_impl = match.group(1)
        elif match.group(2) and current_impl:
            name, params, ret = match.group(2, 3, 4)
            key = f"{current_impl}.{name}"
            start = match.start()
            brace = content.find('{', match.end())
            if brace > 0:
                depth, i = 1, brace + 1
                while i < len(content) and depth > 0:
                    if content[i] == '{': depth += 1
                    elif content[i] == '}': depth -= 1
                    i += 1
                code = content[start:i]
            else:
                code = content[start:start+500]
            clean_params = params.replace('&self, ', '').replace('&self', '').replace('&mut self, ', '').replace('&mut self', '')
            methods[key] = {
                'signature': f"{name}({clean_params})" + (f" -> {ret.strip()}" if ret else ""),
                'code': code[:800].strip(),
            }
    return methods


PARSERS = {'python': parse_python, 'cpp': parse_cpp, 'rust': parse_rust}

# =============================================================================
# API Index - In-memory index of all methods
# =============================================================================

class APIIndex:
    """Index of all Session API methods across languages."""

    def __init__(self):
        self.methods: dict[str, dict[str, dict]] = {}
        self._build()

    def _score_constructor(self, params: str, cls: str) -> int:
        """Score constructor: prefer parameterized over copy/default."""
        is_copy = cls in params and ('const' in params or '&' in params)
        has_primitives = 'double' in params or 'float' in params or 'int' in params
        return 0 if is_copy else (2 if has_primitives else 1)

    def _build(self):
        constructor_scores = {}  # Track best constructor per class per lang
        for lang, files in SOURCE_FILES.items():
            parser = PARSERS[lang]
            for file in files:
                if file.exists():
                    content = file.read_text(errors='ignore')
                    for key, data in parser(content).items():
                        if key not in self.methods:
                            self.methods[key] = {}
                        # For constructors, keep the best one (parameterized over copy)
                        if key.endswith('.constructor') and lang == 'cpp':
                            cls = key.split('.')[0]
                            score_key = f"{lang}:{cls}"
                            new_score = self._score_constructor(data['signature'], cls)
                            old_score = constructor_scores.get(score_key, -1)
                            if new_score <= old_score:
                                continue  # Skip worse constructor
                            constructor_scores[score_key] = new_score
                        self.methods[key][lang] = {**data, 'file': str(file.name)}

    def get_method(self, name: str, language: str = None) -> dict | None:
        """Get method by exact or partial name."""
        langs = [language] if language else None
        if name in self.methods:
            impls = self.methods[name]
            if langs:
                impls = {k: v for k, v in impls.items() if k in langs}
            return {'name': name, 'implementations': impls} if impls else None
        for key, impls in self.methods.items():
            if key.endswith(f".{name}") or key.lower() == name.lower():
                if langs:
                    impls = {k: v for k, v in impls.items() if k in langs}
                return {'name': key, 'implementations': impls} if impls else None
        return None

    def search(self, query: str) -> list[dict]:
        """Search methods by keyword."""
        query_lower = query.lower()
        results = []
        for name, impls in self.methods.items():
            if query_lower in name.lower():
                results.append({'name': name, 'languages': list(impls.keys())})
        return sorted(results, key=lambda x: len(x['name']))[:15]

    def list_class(self, class_name: str) -> list[str]:
        """List all methods of a class."""
        prefix = f"{class_name}."
        return sorted([k for k in self.methods.keys() if k.startswith(prefix)])

    def list_classes(self) -> list[str]:
        """List all indexed classes."""
        return sorted(set(k.split('.')[0] for k in self.methods.keys() if '.' in k))

    def to_browser_format(self) -> dict:
        """Export index in format compatible with Vue browser app."""
        concepts = []
        for name, langs in self.methods.items():
            implementations = {}
            for lang, data in langs.items():
                implementations[lang] = {
                    'sig': data['signature'],
                    'code': data['code'],
                    'file': data.get('file', ''),
                }
            concepts.append({
                'name': name,
                'implementations': implementations,
            })
        return {'concepts': concepts}


# =============================================================================
# MCP Server - FastMCP tools exposed to Claude Code
# =============================================================================

mcp = FastMCP("session-api")
index = APIIndex()


@mcp.tool()
def get_api(method: str, language: str = None) -> str:
    """
    Get Session API method implementation.

    Args:
        method: Method name (e.g., 'Point.distance', 'Line.new', or just 'distance')
        language: Optional filter (python, cpp, rust)

    Returns:
        Method code for all languages or filtered by language.
    """
    result = index.get_method(method, language)
    if not result:
        return f"Method '{method}' not found. Use search_api to find methods."

    output = [f"# {result['name']}\n"]
    lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
    for lang, impl in result['implementations'].items():
        output.append(f"\n## {lang_names[lang]}\n```{lang}\n{impl['code']}\n```")
    return '\n'.join(output)


@mcp.tool()
def search_api(query: str) -> str:
    """
    Search for Session API methods by keyword.

    Args:
        query: Search term (e.g., 'distance', 'transform', 'line')

    Returns:
        List of matching methods with their available languages.
    """
    results = index.search(query)
    if not results:
        return f"No methods found for '{query}'."

    lines = ["Found methods:"]
    for r in results:
        lines.append(f"- {r['name']} ({', '.join(r['languages'])})")
    return '\n'.join(lines)


@mcp.tool()
def list_methods(class_name: str) -> str:
    """
    List all methods of a Session class.

    Args:
        class_name: Class name (e.g., 'Point', 'Line', 'Mesh')

    Returns:
        Comma-separated list of method names.
    """
    methods = index.list_class(class_name)
    if not methods:
        return f"Class '{class_name}' not found. Available: {', '.join(index.list_classes())}"
    return f"{class_name} methods: {', '.join(m.split('.')[-1] for m in methods)}"


@mcp.tool()
def list_classes() -> str:
    """
    List all available Session geometry classes.

    Returns:
        Comma-separated list of class names.
    """
    return f"Classes: {', '.join(index.list_classes())}"


# =============================================================================
# Entry Points
# =============================================================================

def main():
    """Main entry point for MCP server."""
    print(f"Session API Server - {len(index.methods)} methods indexed", file=sys.stderr)
    print(f"Classes: {', '.join(index.list_classes())}", file=sys.stderr)
    mcp.run()


if __name__ == "__main__":
    main()
