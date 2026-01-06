# Session MCP Server

MCP (Model Context Protocol) server exposing Session geometry library API to Claude Code.
Built with [FastMCP](https://github.com/modelcontextprotocol/python-sdk) following official SDK patterns.

## Architecture

```
session_mcp/
├── __init__.py                 # Package exports
├── server.py                   # FastMCP server with 4 tools
├── generate_browser_index.py   # Vue app index generator
├── pyproject.toml              # Package configuration
├── requirements.txt            # Dependencies
└── README.md

.mcp.json                       # Claude Code auto-configuration
```

## Indexed Classes

All 11 geometry classes across Python, C++, and Rust:

| Class | Description |
|-------|-------------|
| Color | RGBA color with presets |
| Line | 3D line segment |
| Mesh | Triangle/quad mesh |
| NurbsCurve | NURBS curve |
| Plane | 3D plane |
| Point | 3D point |
| PointCloud | Point collection |
| Polyline | Connected line segments |
| Tolerance | Numeric tolerance |
| Vector | 3D vector |
| Xform | 4x4 transformation matrix |

## MCP Tools

The server exposes 4 tools to Claude Code:

| Tool | Description | Example |
|------|-------------|---------|
| `get_api` | Get method implementation | `get_api("Line.new")` |
| `search_api` | Search methods by keyword | `search_api("distance")` |
| `list_methods` | List class methods | `list_methods("Point")` |
| `list_classes` | List all classes | `list_classes()` |

## Setup

### 1. Install dependencies

```bash
pip install mcp
```

### 2. Claude Code auto-loads via `.mcp.json`

Restart Claude Code after setup. The server starts automatically.

### 3. Generate Vue browser index (optional)

```bash
python -m session_mcp.generate_browser_index
```

This creates `session_tests/public/apiIndex.js` for Vue app search.

## Usage

### In Claude Code

Ask questions naturally - Claude uses the MCP tools automatically:

```
"how to create a Line in C++"
"what methods does Point have"
"search for distance"
```

### Manual server test

```bash
python -m session_mcp.server
```

## Adding New Classes

1. Add class name to `CLASSES` in `server.py`:

```python
CLASSES = [
    'color',
    'line',
    ...
    'newclass',  # Add here
]
```

2. Ensure source files exist:
   - `session_py/src/session_py/newclass.py`
   - `session_cpp/src/newclass.h` + `newclass.cpp`
   - `session_rust/src/newclass.rs`

3. Regenerate browser index:

```bash
python -m session_mcp.generate_browser_index
```

4. Restart Claude Code.

## How It Works

### MCP Protocol

```
┌─────────────┐     stdio      ┌─────────────────┐
│ Claude Code │ ◄────────────► │ session-mcp     │
│             │   JSON-RPC     │ (FastMCP)       │
└─────────────┘                └─────────────────┘
                                      │
                                      ▼
                              ┌───────────────┐
                              │ Source Files  │
                              │ .py .h .cpp   │
                              │ .rs           │
                              └───────────────┘
```

### FastMCP Pattern

```python
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("session-api")

@mcp.tool()
def get_api(method: str, language: str = None) -> str:
    """Get Session API method implementation."""
    ...

if __name__ == "__main__":
    mcp.run()
```

Key features:
- `@mcp.tool()` decorator registers functions as MCP tools
- Type hints define input schema automatically
- Docstrings become tool descriptions
- `mcp.run()` starts stdio transport

## Vue App Integration

The browser index (`apiIndex.js`) provides client-side search:

```javascript
// window.API_INDEX structure
{
  "concepts": [
    {
      "name": "Point.distance",
      "implementations": {
        "python": { "sig": "distance(...)", "code": "..." },
        "cpp": { "sig": "double distance(...)", "code": "..." },
        "rust": { "sig": "fn distance(...)", "code": "..." }
      }
    }
  ]
}
```

The Vue CLI component (`CliInterface.vue`) uses this for instant search.
