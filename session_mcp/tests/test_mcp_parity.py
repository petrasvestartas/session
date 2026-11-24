"""End-to-end smoke, JSON-RPC 2.0 MCP, `session_py`, and 3-way `.session` parity tests."""

from __future__ import annotations

import json
import pathlib
import subprocess

from session_mcp import ViewerCliClient, create_server, find_cli_binary

SESSION_ROOT = pathlib.Path(__file__).resolve().parents[2]
FLOOR_FIXTURE = SESSION_ROOT / "session_viewer" / "tests" / "fixtures" / "timber-floor.pb"


def test_mcp_rpc_initialize_tools_and_resources(tmp_path: pathlib.Path) -> None:
    with ViewerCliClient() as client:
        srv = create_server(client, force_builtin=True)

        init = srv.handle_rpc({"jsonrpc": "2.0", "id": 1, "method": "initialize"})
        assert init is not None
        assert init["result"]["serverInfo"]["name"] == "session-viewer"

        tools = srv.handle_rpc({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
        names = {t["name"] for t in tools["result"]["tools"]}
        assert {
            "object_create",
            "object_transform",
            "measure",
            "scene_save",
            "scene_open",
            "command_run",
        } <= names

        # Create a 20x30x40 box via JSON-RPC `tools/call`
        call_box = srv.handle_rpc(
            {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {
                    "name": "object_create",
                    "arguments": {
                        "kind": {"type": "box", "origin": [0, 0, 0], "dx": 20, "dy": 30, "dz": 40}
                    },
                },
            }
        )
        assert not call_box["result"]["isError"]
        created_reply = json.loads(call_box["result"]["content"][0]["text"])
        assert len(created_reply["created"]) == 1

        # Read `scene://inspect` and `scene://objects` resources
        insp = srv.handle_rpc(
            {
                "jsonrpc": "2.0",
                "id": 4,
                "method": "resources/read",
                "params": {"uri": "scene://inspect"},
            }
        )
        insp_val = json.loads(insp["result"]["contents"][0]["text"])
        assert insp_val["object_count"] == 1

        obj_path = tmp_path / "box.obj"
        srv.invoke_tool("scene_export_obj", {"path": str(obj_path)})
        assert obj_path.is_file()
        assert "v " in obj_path.read_text()


def test_three_way_byte_identical_session_parity(tmp_path: pathlib.Path) -> None:
    """Command-line verbs, `session-viewer-cli exec`, and FastMCP tools must yield byte-identical `.session` files."""
    cli_bin = find_cli_binary()
    out_cmd = tmp_path / "via_cmd.session"
    out_cli = tmp_path / "via_cli.session"
    out_mcp = tmp_path / "via_mcp.session"

    # Path 1: Command-line verbs (`--cmd`)
    subprocess.run(
        [
            str(cli_bin),
            "exec",
            str(FLOOR_FIXTURE),
            "--cmd",
            "Select All",
            "--cmd",
            "Move 120,40,-15",
            "--cmd",
            "Alpha 0.75",
            "--cmd",
            "Arctic On",
            "--cmd",
            "View Isometric",
            "--save",
            str(out_cmd),
        ],
        check=True,
        capture_output=True,
        text=True,
    )

    # Path 2: Typed `--op` JSON arguments via `session-viewer-cli exec`
    subprocess.run(
        [
            str(cli_bin),
            "exec",
            str(FLOOR_FIXTURE),
            "--op",
            json.dumps({"op": "selection_query", "by": {"type": "all"}}),
            "--op",
            json.dumps(
                {
                    "op": "object_transform",
                    "targets": "selection",
                    "xform": {"type": "translate", "delta": [120.0, 40.0, -15.0]},
                }
            ),
            "--op",
            json.dumps({"op": "alpha_set", "targets": "selection", "alpha": 0.75}),
            "--op",
            json.dumps({"op": "view_set", "patch": {"arctic": True}}),
            "--op",
            json.dumps({"op": "camera_set_view", "view": "iso"}),
            "--save",
            str(out_cli),
        ],
        check=True,
        capture_output=True,
        text=True,
    )

    # Path 3: Python FastMCP server tools
    with ViewerCliClient(binary=cli_bin) as client:
        srv = create_server(client, force_builtin=True)
        srv.invoke_tool("scene_import", {"path": str(FLOOR_FIXTURE)})
        srv.invoke_tool("selection_query", {"by": {"type": "all"}})
        srv.invoke_tool(
            "object_transform",
            {"targets": "selection", "xform": {"type": "translate", "delta": [120.0, 40.0, -15.0]}},
        )
        srv.invoke_tool("alpha_set", {"targets": "selection", "alpha": 0.75})
        srv.invoke_tool("view_set", {"patch": {"arctic": True}})
        srv.invoke_tool("camera_set_view", {"view": "iso"})
        srv.invoke_tool("scene_save", {"path": str(out_mcp)})

    bytes_cmd = out_cmd.read_bytes()
    bytes_cli = out_cli.read_bytes()
    bytes_mcp = out_mcp.read_bytes()
    assert len(bytes_cmd) > 1000
    if bytes_cmd != bytes_cli:
        idx = next((i for i, (a, b) in enumerate(zip(bytes_cmd, bytes_cli)) if a != b), min(len(bytes_cmd), len(bytes_cli)))
        raise AssertionError(
            f"CLI `--op` and `--cmd` .session outputs diverged at byte {idx} "
            f"(len {len(bytes_cmd)} vs {len(bytes_cli)})"
        )
    if bytes_cmd != bytes_mcp:
        idx = next((i for i, (a, b) in enumerate(zip(bytes_cmd, bytes_mcp)) if a != b), min(len(bytes_cmd), len(bytes_mcp)))
        raise AssertionError(
            f"FastMCP and command-line .session outputs diverged at byte {idx} "
            f"(len {len(bytes_cmd)} vs {len(bytes_mcp)})"
        )


def test_session_py_geometry_round_trip_through_mcp(tmp_path: pathlib.Path) -> None:
    from session_py import Mesh, Session

    s = Session("from_python")
    box = Mesh.create_box(50.0, 60.0, 70.0)
    s.add_mesh(box)
    pb_path = tmp_path / "py_box.pb"
    pb_path.write_bytes(s.pb_dumps())

    with ViewerCliClient() as client:
        srv = create_server(client, force_builtin=True)
        imp = srv.invoke_tool("scene_import", {"path": str(pb_path)})
        assert len(imp["created"]) == 1
        ref = imp["created"][0]

        vol = srv.invoke_tool("measure", {"what": {"type": "volume"}, "targets": {"refs": [ref]}})
        assert abs(vol["value"]["volume_approx"] - (50.0 * 60.0 * 70.0)) < 1.0
