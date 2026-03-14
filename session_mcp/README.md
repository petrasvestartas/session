# Session MCP Server

MCP (Model Context Protocol) server exposing Session geometry library API to Claude Code.

## MCP Tools

| Tool | Description | Example |
|------|-------------|---------|
| `get_api` | Get method implementation | `get_api("Line.new")` |
| `search_api` | Search methods by keyword | `search_api("distance")` |
| `list_methods` | List class methods | `list_methods("Point")` |
| `list_classes` | List all classes | `list_classes()` |

## Setup

```bash
pip install mcp
```

Claude Code auto-loads via `.mcp.json`. Restart Claude Code after setup.

## Usage in Claude Code

Ask naturally — Claude uses the MCP tools automatically:

```
"how to create a Line in C++"
"what methods does Point have"
"search for distance"
```

## Adding New Classes

1. Add class name to `CLASSES` in `server.py`
2. Ensure source files exist in `session_py/`, `session_cpp/`, `session_rust/`
3. Regenerate browser index:

```bash
python -m session_mcp.generate_browser_index
```

4. Restart Claude Code
