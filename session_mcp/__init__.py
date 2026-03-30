"""Session MCP Server - Exposes Session geometry API to Claude Code."""

from .server import APIIndex, index

try:
    from .server import mcp, main
except Exception:
    mcp = None
    main = None

__all__ = ['APIIndex', 'index', 'mcp', 'main']
