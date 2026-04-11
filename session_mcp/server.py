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

try:
    from mcp.server.fastmcp import FastMCP
except ImportError:
    FastMCP = None

# =============================================================================
# Configuration
# =============================================================================

REPO_ROOT = Path(__file__).parent.parent

# Files under session_py/src/session_py/ that aren't geometry classes.
_PY_SKIP = {"__init__", "mini_test", "reload"}


def _discover_classes() -> list[str]:
    """Auto-discover the canonical class list from session_py source files.

    One file = one class. Filters tests, scaffolding, and proto stubs.
    """
    py_dir = REPO_ROOT / "session_py" / "src" / "session_py"
    names = set()
    for path in py_dir.glob("*.py"):
        stem = path.stem
        if stem.endswith("_test") or stem in _PY_SKIP:
            continue
        names.add(stem)
    return sorted(names)


CLASSES = _discover_classes()


def _existing(paths):
    return [p for p in paths if p.exists()]


SOURCE_FILES = {
    'python': _existing(REPO_ROOT / f"session_py/src/session_py/{cls}.py" for cls in CLASSES),
    'cpp': _existing(
        f for cls in CLASSES for f in (
            REPO_ROOT / f"session_cpp/src/{cls}.h",
            REPO_ROOT / f"session_cpp/src/{cls}.cpp",
        )
    ),
    'rust': _existing(REPO_ROOT / f"session_rust/src/{cls}.rs" for cls in CLASSES),
}

TEST_FILES = {
    'cpp': _existing(REPO_ROOT / f"session_cpp/src/{cls}_test.cpp" for cls in CLASSES),
    'python': _existing(REPO_ROOT / f"session_py/src/session_py/{cls}_test.py" for cls in CLASSES),
    'rust': _existing(REPO_ROOT / f"session_rust/src/{cls}_test.rs" for cls in CLASSES),
}


def _extract_class_docstrings() -> dict:
    """Extract one-line class docstrings from every Python source file.

    Returns {ClassName: first-line-of-docstring}. Used in place of a hand-
    maintained description dict so summaries don't rot against the code.
    """
    import ast

    out: dict = {}
    for path in SOURCE_FILES['python']:
        try:
            tree = ast.parse(path.read_text(errors='ignore'))
        except SyntaxError:
            continue
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                doc = ast.get_docstring(node)
                if doc:
                    out[node.name] = doc.strip().split('\n', 1)[0].strip()
    return out

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
                lines = content[start:start+8000].split('\n')[:80]
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
                'code': code[:8000].strip(),
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
                    'code': code[:8000],
                }
    # Regular methods: ReturnType ClassName::method(params)
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>,*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)',
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
            'code': code[:8000].strip(),
        }
    # Header declarations (ending with ; or inline { ... })
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>,*&\s]*?)\s+(\w+)\s*\(([^)]*)\)\s*(?:const)?\s*(?:;|\{[^}]*\})',
        content, re.MULTILINE
    ):
        ret, name, params = match.groups()
        if name in ('if', 'return', 'while', 'for', 'switch'):
            continue
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
                'code': code[:8000].strip(),
            }
    return methods


PARSERS = {'python': parse_python, 'cpp': parse_cpp, 'rust': parse_rust}


def parse_cpp_tests(content: str) -> dict:
    """Extract MINI_TEST blocks from C++ test files."""
    tests = {}
    for match in re.finditer(
        r'MINI_TEST\s*\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*\)\s*\{',
        content
    ):
        cls, test_name = match.groups()
        brace_start = match.end() - 1
        depth, i = 1, brace_start + 1
        while i < len(content) and depth > 0:
            if content[i] == '{': depth += 1
            elif content[i] == '}': depth -= 1
            i += 1
        code = content[match.start():i]
        tests[f"{cls}.test_{test_name}"] = {
            'signature': f'MINI_TEST("{cls}", "{test_name}")',
            'code': code[:8000].strip(),
        }
    return tests


def parse_python_tests(content: str) -> dict:
    """Extract @MINI_TEST blocks from Python test files."""
    tests = {}
    for match in re.finditer(
        r'@MINI_TEST\s*\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*\)\s*\ndef\s+(\w+)\s*\(',
        content
    ):
        cls, test_name, func_name = match.groups()
        start = match.start()
        next_test = content.find('@MINI_TEST', match.end())
        next_func = content.find('\ndef ', match.end() + 1)
        end = min(e for e in [next_test, next_func, len(content)] if e > 0)
        code = content[start:end]
        tests[f"{cls}.test_{test_name}"] = {
            'signature': f'@MINI_TEST("{cls}", "{test_name}")',
            'code': code[:8000].strip(),
        }
    return tests


def parse_rust_tests(content: str) -> dict:
    """Extract MINI_TEST! blocks from Rust test files."""
    tests = {}
    for match in re.finditer(
        r'MINI_TEST!\s*\(\s*"(\w+)"\s*,\s*"([^"]+)"\s*,',
        content
    ):
        cls, test_name = match.groups()
        start = match.start()
        depth, i = 0, match.end()
        found_brace = False
        while i < len(content):
            if content[i] == '{':
                depth += 1
                found_brace = True
            elif content[i] == '}':
                depth -= 1
                if found_brace and depth == 0:
                    i += 1
                    break
            i += 1
        # skip closing );
        while i < len(content) and content[i] in ' \t\n\r':
            i += 1
        if i < len(content) and content[i] == ')':
            i += 1
        if i < len(content) and content[i] == ';':
            i += 1
        code = content[start:i]
        tests[f"{cls}.test_{test_name}"] = {
            'signature': f'MINI_TEST!("{cls}", "{test_name}")',
            'code': code[:8000].strip(),
        }
    return tests


TEST_PARSERS = {'cpp': parse_cpp_tests, 'python': parse_python_tests, 'rust': parse_rust_tests}

# =============================================================================
# Recipe Parser - Extract multi-step code examples from web_recipes.md
# =============================================================================

RECIPES_FILE = REPO_ROOT / ".claude" / "skills" / "web_recipes.md"

LANG_MAP = {'cpp': 'cpp', 'python': 'python', 'rust': 'rust'}

def parse_recipes(filepath: Path = RECIPES_FILE) -> list[dict]:
    if not filepath.exists():
        return []
    content = filepath.read_text(errors='ignore')
    recipes = []
    sections = re.split(r'^## ', content, flags=re.MULTILINE)[1:]
    for section in sections:
        lines = section.strip().split('\n')
        title = lines[0].strip()
        if title.startswith('API Quick Reference'):
            continue
        code = {}
        for m in re.finditer(r'\*\*(?:C\+\+|Python|Rust):\*\*\s*\n```(\w+)\n(.*?)```', section, re.DOTALL):
            lang_raw = m.group(1)
            lang = LANG_MAP.get(lang_raw, lang_raw)
            code[lang] = m.group(2).strip()
        if not code:
            continue
        title_lower = title.lower()
        tags = list(set(re.findall(r'[a-z_]+', title_lower)))
        all_code = ' '.join(code.values()).lower()
        for fn in re.findall(r'\.([a-z_]+)\(', all_code):
            if fn not in tags:
                tags.append(fn)
        for cls in re.findall(r'(?:Primitives|NurbsCurve|NurbsSurface|Mesh|Point|Plane|Polyline|Line|Color|Vector|Vertex)\b', ' '.join(code.values())):
            t = cls.lower()
            if t not in tags:
                tags.append(t)
        recipes.append({'title': title, 'tags': tags, 'code': code})
    return recipes

# =============================================================================
# API Index - In-memory index of all methods
# =============================================================================

class APIIndex:
    """Index of all Session API methods across languages."""

    def __init__(self):
        self.methods: dict[str, dict[str, dict]] = {}
        self.tests: dict[str, dict[str, dict]] = {}
        self.recipes: list[dict] = []
        self.related_methods: dict[str, list[str]] = {}
        self.class_summaries: dict[str, str] = {}
        self.class_graph: dict[str, dict] = {}
        self._build()
        self._build_relationships()
        self._build_class_graph()

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
        # Index test files
        for lang, files in TEST_FILES.items():
            parser = TEST_PARSERS[lang]
            for file in files:
                if file.exists():
                    content = file.read_text(errors='ignore')
                    for key, data in parser(content).items():
                        if key not in self.tests:
                            self.tests[key] = {}
                        self.tests[key][lang] = {**data, 'file': str(file.name)}
        # Index recipes
        self.recipes = parse_recipes()

    def _build_relationships(self):
        """Build related_methods and class_summaries from indexed data."""
        # Group methods by class
        class_methods: dict[str, list[str]] = {}
        for key in self.methods:
            if '.' in key:
                cls, method = key.split('.', 1)
                class_methods.setdefault(cls, []).append(method)

        # For each method, find other methods in the same class that reference it
        for key, impls in self.methods.items():
            if '.' not in key:
                continue
            cls, method_name = key.split('.', 1)
            related = set()
            # Check if other methods in the same class reference this method name
            for other_key, other_impls in self.methods.items():
                if other_key == key or not other_key.startswith(f"{cls}."):
                    continue
                other_method = other_key.split('.', 1)[1]
                for lang, data in other_impls.items():
                    if method_name in data.get('code', ''):
                        related.add(other_key)
                        break
            # Also check if this method references other methods
            for lang, data in impls.items():
                code = data.get('code', '')
                for other_method in class_methods.get(cls, []):
                    if other_method != method_name and other_method in code:
                        related.add(f"{cls}.{other_method}")
            if related:
                self.related_methods[key] = sorted(related)

        # Auto-extract class summaries from Python class docstrings.
        # Falls back to a generic description for classes without docstrings.
        auto_descriptions = _extract_class_docstrings()
        for cls in class_methods:
            self.class_summaries[cls] = auto_descriptions.get(
                cls, f'{cls} geometry class'
            )

    def _build_class_graph(self):
        """Build a class-level relationship graph: composition, factories, uses."""
        NOISE = {'fmt', 'std', 'knot'}
        all_classes = sorted(
            {k.split('.', 1)[0] for k in self.methods if '.' in k} - NOISE,
            key=len, reverse=True,
        )
        if not all_classes:
            return
        class_re = re.compile(r'\b(' + '|'.join(re.escape(c) for c in all_classes) + r')\b')

        graph: dict[str, dict] = {
            cls: {'composition': set(), 'factories': set(), 'uses': set()}
            for cls in all_classes
        }

        # uses + factories: scan method signatures
        for key, impls in self.methods.items():
            if '.' not in key:
                continue
            cls, method = key.split('.', 1)
            if cls not in graph:
                continue
            for lang, data in impls.items():
                sig = data.get('signature', '')
                refs = {m for m in class_re.findall(sig) if m != cls}
                graph[cls]['uses'].update(refs)
                if method.startswith('from_') or method == 'from':
                    # cls.from_X(Y) means Y can be converted to cls — edge Y -> cls
                    for ref in refs:
                        if ref in graph:
                            graph[ref]['factories'].add(cls)

        # composition: scan C++ headers line by line for member declarations
        # Heuristic: a member declaration is a line that
        #   - is inside a class (we trust file == class)
        #   - does NOT contain '(' (otherwise it's a method)
        #   - ends with ';' or contains '=' (a field with default value)
        #   - mentions a known class name
        for cls in graph:
            header = REPO_ROOT / f"session_cpp/src/{cls.lower()}.h"
            if not header.exists():
                continue
            content = header.read_text(errors='ignore')
            for raw_line in content.splitlines():
                line = raw_line.strip()
                if not line or line.startswith('//') or line.startswith('*'):
                    continue
                if '(' in line:
                    continue  # method, function call, or function pointer
                if not (line.endswith(';') or '=' in line):
                    continue
                if line.startswith(('return', 'using', 'typedef', '#')):
                    continue
                # Skip forward declarations: `class Foo;`, `struct Foo;`, `friend class Foo;`
                if re.match(r'^(?:friend\s+)?(?:class|struct|enum)\s+\w+\s*;', line):
                    continue
                for ref in class_re.findall(line):
                    if ref != cls:
                        graph[cls]['composition'].add(ref)

        # subtract composition from uses so each edge appears at strongest level only
        for cls, info in graph.items():
            info['uses'] -= info['composition']
            info['uses'] -= info['factories']

        self.class_graph = {
            cls: {
                'composition': sorted(info['composition']),
                'factories': sorted(info['factories']),
                'uses': sorted(info['uses']),
                'summary': self.class_summaries.get(cls, ''),
            }
            for cls, info in graph.items()
        }

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

    def get_test(self, name: str, language: str = None) -> dict | None:
        """Get test by exact or partial name."""
        langs = [language] if language else None
        if name in self.tests:
            impls = self.tests[name]
            if langs:
                impls = {k: v for k, v in impls.items() if k in langs}
            return {'name': name, 'implementations': impls} if impls else None
        for key, impls in self.tests.items():
            if key.endswith(f".{name}") or key.lower() == name.lower():
                if langs:
                    impls = {k: v for k, v in impls.items() if k in langs}
                return {'name': key, 'implementations': impls} if impls else None
        return None

    def search(self, query: str) -> list[dict]:
        """Search methods and tests by keyword."""
        query_lower = query.lower()
        results = []
        for name, impls in self.methods.items():
            if query_lower in name.lower():
                results.append({'name': name, 'languages': list(impls.keys()), 'type': 'method'})
        for name, impls in self.tests.items():
            if query_lower in name.lower():
                results.append({'name': name, 'languages': list(impls.keys()), 'type': 'test'})
        return sorted(results, key=lambda x: len(x['name']))[:20]

    def list_class(self, class_name: str) -> list[str]:
        """List all methods of a class."""
        prefix = f"{class_name}."
        return sorted([k for k in self.methods.keys() if k.startswith(prefix)])

    def list_tests(self, class_name: str) -> list[str]:
        """List all tests for a class."""
        prefix = f"{class_name}."
        return sorted([k for k in self.tests.keys() if k.startswith(prefix)])

    def list_classes(self) -> list[str]:
        """List all indexed classes."""
        return sorted(set(k.split('.')[0] for k in self.methods.keys() if '.' in k))

    def compute_parity_status(self) -> dict:
        """Per-class parity report: which languages cover which methods.

        Returns {ClassName: {cpp, python, rust: count, present_in: [..],
        gaps: int, status: 'DONE'|'TODO'}}. A class is DONE only when every
        method exists in all 3 languages.
        """
        by_class: dict = {}
        for key, impls in self.methods.items():
            if '.' not in key:
                continue
            cls = key.split('.', 1)[0]
            info = by_class.setdefault(
                cls, {'cpp': 0, 'python': 0, 'rust': 0, 'gaps': 0}
            )
            for lang in impls:
                info[lang] = info.get(lang, 0) + 1
            if len(impls) < 3:
                info['gaps'] += 1
        for cls, info in by_class.items():
            present = [l for l in ('cpp', 'python', 'rust') if info[l] > 0]
            info['present_in'] = present
            info['status'] = 'DONE' if len(present) == 3 and info['gaps'] == 0 else 'TODO'
        return by_class

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
            entry = {
                'name': name,
                'implementations': implementations,
            }
            if name in self.related_methods:
                entry['related'] = self.related_methods[name]
            concepts.append(entry)
        for name, langs in self.tests.items():
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

        # Build method_index: maps bare method names to fully qualified names
        method_index = {}
        for name in self.methods:
            if '.' in name:
                bare = name.split('.', 1)[1]
                method_index.setdefault(bare, []).append(name)

        return {
            'concepts': concepts,
            'recipes': self.recipes,
            'class_summaries': self.class_summaries,
            'class_graph': self.class_graph,
            'method_index': method_index,
            'parity_status': self.compute_parity_status(),
        }


# =============================================================================
# MCP Server - FastMCP tools exposed to Claude Code
# =============================================================================

index = APIIndex()

if FastMCP:
    mcp = FastMCP("session-api")

    @mcp.tool()
    def get_api(method: str, language: str = None) -> str:
        """
        Get Session API method implementation and usage examples from tests.

        Args:
            method: Method name (e.g., 'Point.distance', 'Line.new', or just 'distance')
            language: Optional filter (python, cpp, rust)

        Returns:
            Method code for all languages, plus related test examples.
        """
        result = index.get_method(method, language)
        if not result:
            return f"Method '{method}' not found. Use search_api to find methods."

        lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
        output = [f"# {result['name']}\n"]
        for lang, impl in result['implementations'].items():
            output.append(f"\n## {lang_names[lang]}\n```{lang}\n{impl['code']}\n```")

        # Find related tests that use this method
        class_name = result['name'].split('.')[0]
        method_name = result['name'].split('.')[-1]
        related_tests = []
        for test_key, test_impls in index.tests.items():
            if not test_key.startswith(f"{class_name}."):
                continue
            for lang, data in test_impls.items():
                if language and lang != language:
                    continue
                if method_name in data['code']:
                    related_tests.append((test_key, lang, data))
                    break
        if related_tests:
            output.append(f"\n# Usage examples from tests\n")
            for test_key, lang, data in related_tests:
                output.append(f"\n## {test_key} ({lang_names.get(lang, lang)})\n```{lang}\n{data['code']}\n```")
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
    def get_test(test_name: str, language: str = None) -> str:
        """
        Get Session API test implementation (usage examples from _test files).

        Args:
            test_name: Test name (e.g., 'NurbsCurve.test_constructor', 'test_Evaluation')
            language: Optional filter (python, cpp, rust)

        Returns:
            Test code for all languages or filtered by language.
        """
        result = index.get_test(test_name, language)
        if not result:
            return f"Test '{test_name}' not found. Use list_tests to see available tests."

        output = [f"# {result['name']}\n"]
        lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
        for lang, impl in result['implementations'].items():
            ext = {'python': 'python', 'cpp': 'cpp', 'rust': 'rust'}[lang]
            output.append(f"\n## {lang_names[lang]}\n```{ext}\n{impl['code']}\n```")
        return '\n'.join(output)

    @mcp.tool()
    def list_tests(class_name: str) -> str:
        """
        List all tests for a Session class (from _test files).

        Args:
            class_name: Class name (e.g., 'NurbsCurve', 'Point', 'Mesh')

        Returns:
            List of test names with their available languages.
        """
        tests = index.list_tests(class_name)
        if not tests:
            return f"No tests found for '{class_name}'."
        lines = [f"{class_name} tests:"]
        for t in tests:
            langs = list(index.tests[t].keys())
            lines.append(f"  {t.split('.')[1]} ({', '.join(langs)})")
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

    @mcp.tool()
    def get_class(class_name: str, language: str = None) -> str:
        """
        Get the full source file(s) of a Session class.

        Args:
            class_name: Class name (e.g., 'Mesh', 'Point'). Case-insensitive.
            language: Optional filter (python, cpp, rust). If omitted, returns all.

        Returns:
            Full file contents per language. For C++ returns both .h and .cpp.
        """
        cls_lower = class_name.lower()
        lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
        ext_map = {'python': 'py', 'cpp': 'cpp', 'rust': 'rs'}
        out = [f"# {class_name}\n"]
        any_found = False
        for lang, files in SOURCE_FILES.items():
            if language and lang != language:
                continue
            for path in files:
                if path.stem == cls_lower or path.stem.split('.')[0] == cls_lower:
                    out.append(f"\n## {lang_names[lang]} — {path.name}\n```{ext_map[lang]}\n{path.read_text(errors='ignore')}\n```")
                    any_found = True
        if not any_found:
            return f"Class '{class_name}' not found. Use list_classes for available names."
        return '\n'.join(out)

    @mcp.tool()
    def get_signature(method: str, language: str = None) -> str:
        """
        Get just the signature(s) of a method, no body. Cheap context for Claude.

        Args:
            method: Method name (e.g., 'Point.distance' or 'distance').
            language: Optional filter (python, cpp, rust).

        Returns:
            One signature per language, no implementation body.
        """
        result = index.get_method(method, language)
        if not result:
            return f"Method '{method}' not found."
        lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
        out = [f"# {result['name']}"]
        for lang, impl in result['implementations'].items():
            out.append(f"- **{lang_names[lang]}**: `{impl['sig'] if 'sig' in impl else impl.get('signature', '')}`")
        return '\n'.join(out)

    @mcp.tool()
    def compare_languages(method: str) -> str:
        """
        Show the same method side-by-side across Python, C++, and Rust.

        Use this for cross-language parity review: C++ is the ground truth,
        Python and Rust must match its API and semantics.

        Args:
            method: Method name (e.g., 'Mesh.weld_vertices' or 'weld_vertices').

        Returns:
            Markdown with one code block per language, plus a parity summary.
        """
        result = index.get_method(method)
        if not result:
            return f"Method '{method}' not found."
        impls = result['implementations']
        present = sorted(impls.keys())
        missing = [l for l in ('cpp', 'python', 'rust') if l not in impls]
        lang_names = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}
        out = [f"# {result['name']}", f"**Present in:** {', '.join(lang_names[l] for l in present)}"]
        if missing:
            out.append(f"**Missing in:** {', '.join(lang_names[l] for l in missing)}")
        for lang in ('cpp', 'python', 'rust'):
            if lang not in impls:
                continue
            out.append(f"\n## {lang_names[lang]}\n```{lang}\n{impls[lang]['code']}\n```")
        return '\n'.join(out)

    @mcp.tool()
    def list_missing(class_name: str = None) -> str:
        """
        Report methods that exist in some languages but not others (parity gaps).

        C++ is the ground truth. A method present in C++ but missing in Python
        or Rust is a port-debt item.

        Args:
            class_name: Optional class to scope the report. If omitted, scans all.

        Returns:
            Per-class list of methods with which language(s) are missing.
        """
        from collections import defaultdict
        gaps: dict[str, list[tuple[str, list[str]]]] = defaultdict(list)
        for key, impls in index.methods.items():
            if '.' not in key:
                continue
            cls, method = key.split('.', 1)
            if class_name and cls.lower() != class_name.lower():
                continue
            present = set(impls.keys())
            if present == {'cpp', 'python', 'rust'}:
                continue
            missing = sorted({'cpp', 'python', 'rust'} - present)
            gaps[cls].append((method, missing))
        if not gaps:
            scope = f" for {class_name}" if class_name else ""
            return f"No parity gaps{scope} — every method exists in all 3 languages."
        out = []
        for cls in sorted(gaps):
            out.append(f"\n## {cls}")
            for method, missing in sorted(gaps[cls]):
                out.append(f"- `{method}` — missing in: {', '.join(missing)}")
        return '\n'.join(out)

    def main():
        """Main entry point for MCP server."""
        print(f"Session API Server - {len(index.methods)} methods, {len(index.tests)} tests indexed", file=sys.stderr)
        print(f"Classes: {', '.join(index.list_classes())}", file=sys.stderr)
        mcp.run()
else:
    mcp = None
    main = None


if __name__ == "__main__":
    if main is None:
        sys.exit(
            "session_mcp.server: FastMCP is not installed in this interpreter. "
            "Run: uv pip install -e session_mcp"
        )
    main()
