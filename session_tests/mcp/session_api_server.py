#!/usr/bin/env python3
"""
Session API MCP Server

Exposes Session library API (Point, Color, etc.) to Claude via MCP.
Claude can query for methods across Python, C++, and Rust.

Usage:
    python session_api_server.py

Add to Claude Desktop config (~/.config/claude/claude_desktop_config.json):
{
    "mcpServers": {
        "session-api": {
            "command": "python",
            "args": ["/path/to/session_api_server.py"]
        }
    }
}
"""

import json
import re
import sys
from pathlib import Path
from typing import Optional
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import Tool, TextContent

# =============================================================================
# Config
# =============================================================================

REPO_ROOT = Path(__file__).parent.parent.parent  # session/
LANGUAGES = ['python', 'cpp', 'rust']

SOURCE_FILES = {
    'python': [
        REPO_ROOT / "session_py/src/session_py/point.py",
        REPO_ROOT / "session_py/src/session_py/color.py",
        REPO_ROOT / "session_py/src/session_py/vector.py",
    ],
    'cpp': [
        REPO_ROOT / "session_cpp/src/point.h",
        REPO_ROOT / "session_cpp/src/point.cpp",
        REPO_ROOT / "session_cpp/src/color.h",
        REPO_ROOT / "session_cpp/src/color.cpp",
        REPO_ROOT / "session_cpp/src/vector.h",
        REPO_ROOT / "session_cpp/src/vector.cpp",
    ],
    'rust': [
        REPO_ROOT / "session_rust/src/point.rs",
        REPO_ROOT / "session_rust/src/color.rs",
        REPO_ROOT / "session_rust/src/vector.rs",
    ],
}

# =============================================================================
# Simple Parsers (extract method signatures + code)
# =============================================================================

def parse_python(content: str) -> dict:
    """Extract class methods from Python"""
    methods = {}
    current_class = None
    
    for match in re.finditer(
        r'^class\s+(\w+)|^(\s*)def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:',
        content, re.MULTILINE
    ):
        if match.group(1):  # class
            current_class = match.group(1)
        elif match.group(3):  # method/function
            indent, name, params, ret = match.group(2, 3, 4, 5)
            if current_class and indent:
                key = f"{current_class}.{name}"
                # Get method body (next 20 lines)
                start = match.end()
                lines = content[start:start+1500].split('\n')[:20]
                code = match.group(0) + '\n' + '\n'.join(lines)
                
                methods[key] = {
                    'signature': f"{name}({params.replace('self, ', '').replace('self', '')})" + (f" -> {ret}" if ret else ""),
                    'code': code.strip(),
                }
    return methods


def parse_cpp(content: str) -> dict:
    """Extract class methods from C++"""
    methods = {}
    
    # Header declarations: ReturnType ClassName::method(params)
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)::(\w+)\s*\(([^)]*)\)',
        content, re.MULTILINE
    ):
        ret, cls, name, params = match.groups()
        key = f"{cls}.{name}"
        
        # Get function body
        start = match.start()
        end = content.find('}', match.end())
        code = content[start:end+1] if end > 0 else content[start:start+500]
        
        methods[key] = {
            'signature': f"{ret.strip()} {name}({params})",
            'code': code[:800].strip(),
        }
    
    # Also get header-only declarations
    for match in re.finditer(
        r'^\s*(?:static\s+)?([a-zA-Z_][\w:<>*&\s]*?)\s+(\w+)\s*\(([^)]*)\)\s*(?:const)?\s*;',
        content, re.MULTILINE
    ):
        ret, name, params = match.groups()
        # Try to find class context
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
    """Extract impl methods from Rust"""
    methods = {}
    current_impl = None
    
    for match in re.finditer(
        r'^impl(?:<[^>]+>)?\s+(\w+)|^\s*pub\s+fn\s+(\w+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)(?:\s*->\s*([^\{]+))?',
        content, re.MULTILINE
    ):
        if match.group(1):  # impl block
            current_impl = match.group(1)
        elif match.group(2) and current_impl:  # method
            name, params, ret = match.group(2, 3, 4)
            key = f"{current_impl}.{name}"
            
            # Get method body
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
# API Index (built on startup)
# =============================================================================

class APIIndex:
    """Indexes all Session API methods across languages"""
    
    def __init__(self):
        self.methods: dict[str, dict[str, dict]] = {}  # method_name -> {lang -> {sig, code}}
        self._build()
    
    def _build(self):
        """Parse all source files"""
        for lang, files in SOURCE_FILES.items():
            parser = PARSERS[lang]
            for file in files:
                if file.exists():
                    content = file.read_text(errors='ignore')
                    for key, data in parser(content).items():
                        if key not in self.methods:
                            self.methods[key] = {}
                        self.methods[key][lang] = {**data, 'file': str(file.name)}
    
    def get_method(self, name: str, languages: list[str] = None) -> Optional[dict]:
        """Get method implementations"""
        # Exact match
        if name in self.methods:
            impls = self.methods[name]
            if languages:
                impls = {k: v for k, v in impls.items() if k in languages}
            return {'name': name, 'implementations': impls} if impls else None
        
        # Partial match (e.g., "distance" matches "Point.distance")
        for key, impls in self.methods.items():
            if key.endswith(f".{name}") or key.lower() == name.lower():
                if languages:
                    impls = {k: v for k, v in impls.items() if k in languages}
                return {'name': key, 'implementations': impls} if impls else None
        return None
    
    def search(self, query: str, languages: list[str] = None) -> list[dict]:
        """Search for methods matching query"""
        query_lower = query.lower()
        results = []
        
        for name, impls in self.methods.items():
            if query_lower in name.lower():
                if languages:
                    impls = {k: v for k, v in impls.items() if k in languages}
                if impls:
                    results.append({'name': name, 'languages': list(impls.keys())})
        
        return sorted(results, key=lambda x: len(x['name']))[:10]
    
    def list_class(self, class_name: str) -> list[str]:
        """List all methods of a class"""
        prefix = f"{class_name}."
        return sorted([k for k in self.methods.keys() if k.startswith(prefix)])
    
    def list_classes(self) -> list[str]:
        """List all classes"""
        return sorted(set(k.split('.')[0] for k in self.methods.keys() if '.' in k))


# =============================================================================
# MCP Server
# =============================================================================

app = Server("session-api")
index = APIIndex()


@app.list_tools()
async def list_tools():
    return [
        Tool(
            name="get_api",
            description="Get Session API method with code for all languages (Python, C++, Rust). Use for 'how to X' questions.",
            inputSchema={
                "type": "object",
                "properties": {
                    "method": {"type": "string", "description": "Method name, e.g., 'Point.distance' or just 'distance'"},
                    "language": {"type": "string", "description": "Optional: filter to one language (python/cpp/rust)"},
                },
                "required": ["method"]
            }
        ),
        Tool(
            name="search_api",
            description="Search for Session API methods by keyword",
            inputSchema={
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Search query, e.g., 'distance', 'color', 'transform'"},
                },
                "required": ["query"]
            }
        ),
        Tool(
            name="list_methods",
            description="List all methods of a Session class",
            inputSchema={
                "type": "object",
                "properties": {
                    "class_name": {"type": "string", "description": "Class name, e.g., 'Point' or 'Color'"},
                },
                "required": ["class_name"]
            }
        ),
        Tool(
            name="list_classes",
            description="List all available Session classes",
            inputSchema={"type": "object", "properties": {}}
        ),
    ]


@app.call_tool()
async def call_tool(name: str, arguments: dict):
    if name == "get_api":
        method = arguments["method"]
        lang = arguments.get("language")
        langs = [lang] if lang else None
        
        result = index.get_method(method, langs)
        if not result:
            return [TextContent(type="text", text=f"Method '{method}' not found. Try search_api.")]
        
        # Format nicely
        output = [f"# {result['name']}\n"]
        for lang, impl in result['implementations'].items():
            lang_name = {'python': 'Python', 'cpp': 'C++', 'rust': 'Rust'}[lang]
            output.append(f"\n## {lang_name}\n```{lang}\n{impl['code']}\n```")
        
        return [TextContent(type="text", text='\n'.join(output))]
    
    elif name == "search_api":
        results = index.search(arguments["query"])
        if not results:
            return [TextContent(type="text", text="No methods found.")]
        
        output = "Found methods:\n" + '\n'.join(
            f"- {r['name']} ({', '.join(r['languages'])})" for r in results
        )
        return [TextContent(type="text", text=output)]
    
    elif name == "list_methods":
        methods = index.list_class(arguments["class_name"])
        if not methods:
            return [TextContent(type="text", text=f"Class '{arguments['class_name']}' not found.")]
        return [TextContent(type="text", text=f"Methods: {', '.join(m.split('.')[-1] for m in methods)}")]
    
    elif name == "list_classes":
        classes = index.list_classes()
        return [TextContent(type="text", text=f"Classes: {', '.join(classes)}")]
    
    return [TextContent(type="text", text=f"Unknown tool: {name}")]


async def main():
    async with stdio_server() as (read, write):
        await app.run(read, write, app.create_initialization_options())


if __name__ == "__main__":
    import asyncio
    print(f"Session API MCP Server - Indexed {len(index.methods)} methods", file=sys.stderr)
    print(f"Classes: {', '.join(index.list_classes())}", file=sys.stderr)
    asyncio.run(main())
