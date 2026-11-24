# `session-mcp` — FastMCP Server for Session Viewer

`session_mcp` exposes the Session Viewer shared `viewer_core::op::Op` API as Model Context Protocol (MCP) tools and resources over `session-viewer-cli serve --stdio`.

- **Single geometry engine:** All scene mutations, measurements, layer edits, and `.session` / `.obj` / `.pb` file operations execute inside Rust (`session_rust` + `viewer-core`), guaranteeing byte-for-byte parity with the interactive command dock and `session-viewer-cli`.
- **FastMCP + zero-dependency fallback:** Uses `fastmcp.FastMCP` or `mcp.server.fastmcp.FastMCP` when installed, and includes a built-in JSON-RPC 2.0 MCP stdio server so `python -m session_mcp.server` works offline in minimal environments as well.

## Quick Start

```bash
# Run MCP server over stdio
PYTHONPATH=session/session_mcp/src python3 -m session_mcp.server
```
