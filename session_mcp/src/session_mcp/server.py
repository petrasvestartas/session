"""FastMCP server wrapping `ViewerCliClient` with typed MCP tools and resources."""

from __future__ import annotations

import inspect
import json
import sys
from typing import Any, Callable

from .client import ViewerCliClient, ViewerOpError

try:
    from fastmcp import FastMCP as _UpstreamFastMCP  # type: ignore[import-not-found]
except ImportError:
    try:
        from mcp.server.fastmcp import FastMCP as _UpstreamFastMCP  # type: ignore[import-not-found]
    except ImportError:
        _UpstreamFastMCP = None


class _BuiltinFastMCP:
    """Zero-dependency MCP JSON-RPC 2.0 stdio server matching FastMCP's decorator API."""

    def __init__(self, name: str) -> None:
        self.name = name
        self._tools: dict[str, tuple[Callable[..., Any], dict[str, Any]]] = {}
        self._resources: dict[str, Callable[[], str]] = {}

    def tool(self, name: str | None = None) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        def decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            tool_name = name or fn.__name__
            sig = inspect.signature(fn)
            props: dict[str, Any] = {}
            required: list[str] = []
            for param_name, param in sig.parameters.items():
                props[param_name] = {"type": _json_type(param.annotation)}
                if param.default is inspect.Parameter.empty:
                    required.append(param_name)
            schema = {
                "name": tool_name,
                "description": (inspect.getdoc(fn) or "").strip(),
                "inputSchema": {
                    "type": "object",
                    "properties": props,
                    "required": required,
                },
            }
            self._tools[tool_name] = (fn, schema)
            return fn

        return decorator

    def resource(self, uri: str) -> Callable[[Callable[[], str]], Callable[[], str]]:
        def decorator(fn: Callable[[], str]) -> Callable[[], str]:
            self._resources[uri] = fn
            return fn

        return decorator

    def list_tool_schemas(self) -> list[dict[str, Any]]:
        return [schema for _, schema in self._tools.values()]

    def list_resource_uris(self) -> list[str]:
        return list(self._resources.keys())

    def invoke_tool(self, name: str, arguments: dict[str, Any] | None = None) -> dict[str, Any]:
        if name not in self._tools:
            raise KeyError(f"unknown tool: {name}")
        fn, _ = self._tools[name]
        return fn(**(arguments or {}))

    def invoke_resource(self, uri: str) -> str:
        if uri not in self._resources:
            raise KeyError(f"unknown resource: {uri}")
        return self._resources[uri]()

    def handle_rpc(self, msg: dict[str, Any]) -> dict[str, Any] | None:
        method = msg.get("method")
        req_id = msg.get("id")
        params = msg.get("params") or {}
        if req_id is None:
            return None
        try:
            if method == "initialize":
                result: Any = {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {}, "resources": {}},
                    "serverInfo": {"name": self.name, "version": "0.1.0"},
                }
            elif method == "tools/list":
                result = {"tools": self.list_tool_schemas()}
            elif method == "tools/call":
                tname = params["name"]
                targs = params.get("arguments") or {}
                try:
                    out = self.invoke_tool(tname, targs)
                    result = {
                        "content": [{"type": "text", "text": json.dumps(out)}],
                        "isError": False,
                    }
                except ViewerOpError as err:
                    result = {
                        "content": [
                            {
                                "type": "text",
                                "text": json.dumps({"code": err.code, "message": err.op_message}),
                            }
                        ],
                        "isError": True,
                    }
            elif method == "resources/list":
                result = {
                    "resources": [
                        {"uri": uri, "name": uri, "mimeType": "application/json"}
                        for uri in self.list_resource_uris()
                    ]
                }
            elif method == "resources/read":
                uri = params["uri"]
                text = self.invoke_resource(uri)
                result = {
                    "contents": [{"uri": uri, "mimeType": "application/json", "text": text}]
                }
            elif method == "ping":
                result = {}
            else:
                return {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": {"code": -32601, "message": f"method not found: {method}"},
                }
            return {"jsonrpc": "2.0", "id": req_id, "result": result}
        except Exception as exc:  # noqa: BLE001
            return {
                "jsonrpc": "2.0",
                "id": req_id,
                "error": {"code": -32603, "message": str(exc)},
            }

    def run(self) -> None:
        for line in sys.stdin:
            stripped = line.strip()
            if not stripped:
                continue
            resp = self.handle_rpc(json.loads(stripped))
            if resp is not None:
                sys.stdout.write(json.dumps(resp) + "\n")
                sys.stdout.flush()


def _json_type(annotation: Any) -> str:
    text = str(annotation)
    if "bool" in text:
        return "boolean"
    if "int" in text:
        return "integer"
    if "float" in text:
        return "number"
    if "list" in text:
        return "array"
    if "dict" in text:
        return "object"
    return "string"


def create_server(client: ViewerCliClient | None = None, *, force_builtin: bool = False) -> Any:
    """Create the `session-viewer` FastMCP server bound to a `ViewerCliClient`."""
    own_client = client or ViewerCliClient()
    mcp_cls = _BuiltinFastMCP if (force_builtin or _UpstreamFastMCP is None) else _UpstreamFastMCP
    mcp = mcp_cls("session-viewer")
    mcp.viewer_client = own_client  # type: ignore[attr-defined]

    @mcp.tool()
    def object_create(kind: dict[str, Any], layer: dict[str, Any] | None = None) -> dict[str, Any]:
        """Create a point, line, arrow, polyline, curve, circle, box, sphere, cylinder, cone, torus, or text."""
        return own_client.execute("object_create", kind=kind, layer=layer)

    @mcp.tool()
    def object_delete(targets: Any = "selection") -> dict[str, Any]:
        """Delete selected objects or explicit `{"refs": [{"doc": 0, "guid": "..."}]}`."""
        return own_client.execute("object_delete", targets=targets)

    @mcp.tool()
    def object_transform(xform: dict[str, Any], targets: Any = "selection") -> dict[str, Any]:
        """Transform target objects (`translate`, `rotate` with explicit `pivot`, `scale`, `orient3`, `matrix`)."""
        return own_client.execute("object_transform", targets=targets, xform=xform)

    @mcp.tool()
    def object_copy(xform: dict[str, Any], targets: Any = "selection") -> dict[str, Any]:
        """Duplicate target objects transformed by `xform`."""
        return own_client.execute("object_copy", targets=targets, xform=xform)

    @mcp.tool()
    def object_explode(targets: Any = "selection") -> dict[str, Any]:
        """Explode polylines, BReps, meshes, or point clouds into constituent parts."""
        return own_client.execute("object_explode", targets=targets)

    @mcp.tool()
    def object_project(plane: dict[str, Any], targets: Any = "selection") -> dict[str, Any]:
        """Project target objects onto `xy`, `yz`, `zx`, `c_plane`, `points`, or `normal`."""
        return own_client.execute("object_project", targets=targets, plane=plane)

    @mcp.tool()
    def curve_interval(
        target: dict[str, Any], t0: float, t1: float, extend: bool = False
    ) -> dict[str, Any]:
        """Trim or extend a curve over `[t0, t1]`."""
        return own_client.execute("curve_interval", target=target, t0=t0, t1=t1, extend=extend)

    @mcp.tool()
    def curve_arrowhead(head: str, targets: Any = "selection") -> dict[str, Any]:
        """Set curve arrowheads (`none`, `start`, `end`, `both`)."""
        return own_client.execute("curve_arrowhead", targets=targets, head=head)

    @mcp.tool()
    def selection_set(targets: Any, mode: str = "replace") -> dict[str, Any]:
        """Set, add to, or remove from selection by `(doc, guid)` refs."""
        return own_client.execute("selection_set", targets=targets, mode=mode)

    @mcp.tool()
    def selection_query(by: dict[str, Any]) -> dict[str, Any]:
        """Select objects matching `all`, `name`, `smaller_than`, `in_box`, or `kind`."""
        return own_client.execute("selection_query", by=by)

    @mcp.tool()
    def selection_clear() -> dict[str, Any]:
        """Clear the current selection."""
        return own_client.execute("selection_clear")

    @mcp.tool()
    def layer_list() -> dict[str, Any]:
        """List all layers across open documents."""
        return own_client.execute("layer_list")

    @mcp.tool()
    def layer_create(
        name: str | None = None,
        parent: dict[str, Any] | None = None,
        sublayer: bool = False,
    ) -> dict[str, Any]:
        """Create a new layer or sublayer."""
        return own_client.execute("layer_create", name=name, parent=parent, sublayer=sublayer)

    @mcp.tool()
    def layer_assign(layer: dict[str, Any], targets: Any = "selection") -> dict[str, Any]:
        """Move target objects onto `layer` (`{"doc": 0, "name": "..."}`)."""
        return own_client.execute("layer_assign", targets=targets, layer=layer)

    @mcp.tool()
    def visibility_set(visible: bool, targets: Any = "selection") -> dict[str, Any]:
        """Hide or show target objects (undoable)."""
        return own_client.execute("visibility_set", targets=targets, visible=visible)

    @mcp.tool()
    def show_all() -> dict[str, Any]:
        """Unhide all hidden objects (undoable)."""
        return own_client.execute("show_all")

    @mcp.tool()
    def lock_set(locked: bool, targets: Any = "selection") -> dict[str, Any]:
        """Lock or unlock target objects (undoable)."""
        return own_client.execute("lock_set", targets=targets, locked=locked)

    @mcp.tool()
    def color_set(
        color: list[int] | None, edge: bool = False, targets: Any = "selection"
    ) -> dict[str, Any]:
        """Set or clear `[r, g, b]` face or edge colour override (undoable)."""
        return own_client.execute("color_set", targets=targets, edge=edge, color=color)

    @mcp.tool()
    def alpha_set(alpha: float | None, targets: Any = "selection") -> dict[str, Any]:
        """Set (`0.0..=1.0`) or reset (`None`) per-object alpha transparency (undoable)."""
        return own_client.execute("alpha_set", targets=targets, alpha=alpha)

    @mcp.tool()
    def scene_inspect() -> dict[str, Any]:
        """Inspect scene counts, bounds, selection, camera, and view flags."""
        return own_client.execute("scene_inspect")

    @mcp.tool()
    def object_list(filter: dict[str, Any] | None = None) -> dict[str, Any]:
        """List objects with `(doc, guid)` refs, kinds, layers, and bounds."""
        return own_client.execute("object_list", filter=filter)

    @mcp.tool()
    def object_get(target: dict[str, Any]) -> dict[str, Any]:
        """Inspect one object by `{"doc": 0, "guid": "..."}`."""
        return own_client.execute("object_get", target=target)

    @mcp.tool()
    def measure(what: dict[str, Any], targets: Any = "selection") -> dict[str, Any]:
        """Measure `length`, `area`, `volume`, `bounds_extent`, or `distance`."""
        return own_client.execute("measure", what=what, targets=targets)

    @mcp.tool()
    def scene_open(path: str) -> dict[str, Any]:
        """Open a `.session` file from disk."""
        return own_client.execute("scene_open", path=path)

    @mcp.tool()
    def scene_save(path: str) -> dict[str, Any]:
        """Save the complete scene, camera, and display settings to a `.session` file."""
        return own_client.execute("scene_save", path=path)

    @mcp.tool()
    def scene_load_manifest(path: str) -> dict[str, Any]:
        """Load a `.yaml`/`.json`/`.pb` scene manifest or protobuf file."""
        return own_client.execute("scene_load_manifest", path=path)

    @mcp.tool()
    def scene_import(path: str) -> dict[str, Any]:
        """Import a `.pb`, `.obj`, or `.session` file into the current scene."""
        return own_client.execute("scene_import", path=path)

    @mcp.tool()
    def scene_export_obj(path: str) -> dict[str, Any]:
        """Export selected or all mesh/BRep/surface/element objects to Wavefront `.obj`."""
        return own_client.execute("scene_export_obj", path=path)

    @mcp.tool()
    def history_undo() -> dict[str, Any]:
        """Undo the last edit."""
        return own_client.execute("history_undo")

    @mcp.tool()
    def history_redo() -> dict[str, Any]:
        """Redo the last undone edit."""
        return own_client.execute("history_redo")

    @mcp.tool()
    def camera_set_view(view: str) -> dict[str, Any]:
        """Set camera view (`top`, `bottom`, `front`, `back`, `left`, `right`, `iso`, `isoback`)."""
        return own_client.execute("camera_set_view", view=view)

    @mcp.tool()
    def camera_set_pose(
        target: list[float], position: list[float], perspective: bool | None = None
    ) -> dict[str, Any]:
        """Set explicit camera target and position in scene units."""
        return own_client.execute(
            "camera_set_pose", target=target, position=position, perspective=perspective
        )

    @mcp.tool()
    def camera_fit(targets: Any | None = None) -> dict[str, Any]:
        """Fit camera to targets, selection, or whole scene."""
        return own_client.execute("camera_fit", targets=targets)

    @mcp.tool()
    def view_set(patch: dict[str, Any]) -> dict[str, Any]:
        """Update viewport display settings (`arctic`, `wireframe`, `opacity`, `cull_backfaces`, etc.)."""
        return own_client.execute("view_set", patch=patch)

    @mcp.tool()
    def clipping_plane_add(plane: dict[str, Any]) -> dict[str, Any]:
        """Create a clipping plane (`xy`, `yz`, `zx`, `points`, or `normal`)."""
        return own_client.execute("clipping_plane_add", plane=plane)

    @mcp.tool()
    def render_frame(path: str, width: int = 900, height: int = 700) -> dict[str, Any]:
        """Render the current headless scene to a `.ppm` image."""
        return own_client.execute("render_frame", width=width, height=height, path=path)

    @mcp.tool()
    def command_run(line: str) -> dict[str, Any]:
        """Execute any Session Viewer command-line string."""
        return own_client.execute("command_run", line=line)

    @mcp.resource("scene://inspect")
    def resource_inspect() -> str:
        return json.dumps(own_client.execute("scene_inspect")["value"])

    @mcp.resource("scene://objects")
    def resource_objects() -> str:
        return json.dumps(own_client.execute("object_list")["value"])

    @mcp.resource("scene://perf")
    def resource_perf() -> str:
        return json.dumps(own_client.execute("perf_stats")["value"])

    return mcp


def main() -> None:
    with ViewerCliClient() as client:
        server = create_server(client)
        server.run()


if __name__ == "__main__":
    main()
