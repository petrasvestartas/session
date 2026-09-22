"""Exercise the real publication scripts with local storage and failure injection."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parents[2] / "bash"
MOCK = '''#!/usr/bin/env python3
import os, pathlib, shutil, sys
args = sys.argv[1:]
root = pathlib.Path(os.environ["MOCK_ROOT"])
url = args[-1]
key = url.split("/session-viewer-data/")[-1] if "/session-viewer-data/" in url else url.split("https://mock/")[-1]
path = root / "objects" / key
if "-T" in args:
    config = pathlib.Path(args[args.index("-K")+1])
    assert config.exists() and config.stat().st_mode & 0o077 == 0
    with (root / "events").open("a") as events:
        events.write("put " + key + "\\n")
    if os.environ.get("MOCK_FAIL") == "put" and "/revisions/" in key:
        print("500", end="")
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(args[args.index("-T")+1], path)
        print("200", end="")
elif "-X" in args and "PUT" in args:
    header = next(value for value in args if value.startswith("x-amz-copy-source: "))
    source = root / "objects" / header.split("/session-viewer-data/")[-1]
    path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, path)
    with (root / "events").open("a") as events:
        events.write("copy " + key + "\\n")
    pathlib.Path(args[args.index("-o")+1]).write_text("<CopyObjectResult/>")
    print("200", end="")
elif "-I" in args:
    print("200" if path.exists() else "404", end="")
else:
    if path.exists() and not (os.environ.get("MOCK_FAIL") == "verify" and "/revisions/" in key):
        print("HTTP/1.1 200 OK\\nContent-Length: " + str(path.stat().st_size))
    else:
        print("HTTP/1.1 404 Not Found")
'''


class Publication(unittest.TestCase):
    """A manifest cannot become visible before its immutable payload passes verification."""
    def run_publish(self, failure="", repeat=False):
        directory = tempfile.TemporaryDirectory(prefix="viewer-publication-")
        self.addCleanup(directory.cleanup)
        root = Path(directory.name)
        (root / "curl").write_text(MOCK)
        (root / "curl").chmod(0o700)
        (root / "scan.pb").write_bytes(b"test-payload-v2")
        (root / "scene.toml").write_text('name="test"\n[[items]]\nfile="scan.pb"\nat=[1,2,3]\npoint_size=4\n[[texts]]\ntext="text_not_oriented_to_camera"\nat=[23400,-420,60]\nright=[1,0,0]\nup=[0,1,0]\nheight=32\n')
        (root / "config").write_text('user="test:test"\n')
        (root / "config").chmod(0o600)
        env = dict(os.environ, PATH=str(root) + os.pathsep + os.environ["PATH"], MOCK_ROOT=str(root), MOCK_FAIL=failure,
                   R2_ENDPOINT="https://mock", R2_PUBLIC="https://mock", R2_CURL_CONFIG=str(root / "config"), R2_NOTIFY_ENABLED="0")
        # Only credential lookup is replaced; the real publication helpers run unchanged.
        local = root / "bash"
        import shutil
        shutil.copytree(SCRIPTS, local)
        helper = local / "lib/view.sh"
        helper.write_text(helper.read_text() + '\nr2_require_credentials() { return 0; }\n')
        result = subprocess.run(["bash", str(local / "view_live.sh"), str(root / "scene.toml"), str(root / "scan.pb")], env=env, text=True, capture_output=True)
        if repeat and result.returncode == 0:
            (root / "config").write_text('user="test:test"\n')
            (root / "config").chmod(0o600)
            result = subprocess.run(["bash", str(local / "view_live.sh"), str(root / "scene.toml"), str(root / "scan.pb")], env=env, text=True, capture_output=True)
        events = (root / "events").read_text().splitlines() if (root / "events").exists() else []
        return root, result, events

    def test_toml_preserves_placement_and_publishes_verified_revision_first(self):
        import json
        import tomllib
        root, result, events = self.run_publish()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        self.assertTrue(events[0].startswith("put pb/revisions/"))
        self.assertEqual(events[1:], ["copy pb/view_live.pb", "put scenes/view_live.toml", "put scenes/view_live.yaml"])
        manifest = tomllib.loads((root / "objects/scenes/view_live.toml").read_text())
        self.assertEqual(manifest["items"][0]["at"], [1, 2, 3])
        self.assertTrue(manifest["items"][0]["file"].startswith("pb/revisions/"))
        self.assertEqual(manifest["texts"], [{"text": "text_not_oriented_to_camera", "at": [23400,-420,60], "right": [1,0,0], "up": [0,1,0], "height": 32}])
        self.assertEqual(manifest, json.loads((root / "objects/scenes/view_live.yaml").read_text()))

    def test_failed_payload_put_leaves_manifests_untouched(self):
        _, result, events = self.run_publish("put")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(any("scenes/" in event for event in events))

    def test_failed_payload_verification_leaves_manifests_untouched(self):
        _, result, events = self.run_publish("verify")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(any("scenes/" in event for event in events))

    def test_unchanged_revision_reuses_payload_without_a_second_upload(self):
        _, result, events = self.run_publish(repeat=True)
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        self.assertEqual(sum(event.startswith("put pb/revisions/") for event in events), 1)
        self.assertEqual(events.count("put scenes/view_live.toml"), 2)


if __name__ == "__main__":
    unittest.main()
# Viewer directory: python3 tests/publication.py. No remote writes, messages or credentials used.
