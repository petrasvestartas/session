import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('view_manifest.py')
WORK = Path(os.environ.get('VIEWER_REVIEW_WORK', str(Path.home() / 'viewer_review_work')))
WORK.mkdir(exist_ok=True)

class ManifestTests(unittest.TestCase):
    def test_encoding_size_and_authored_fields_survive(self):
        with tempfile.TemporaryDirectory(dir=WORK) as directory:
            root = Path(directory)
            geometry = root / 'floor.pb'
            geometry.write_bytes(b'original protobuf bytes')
            manifest = root / 'scene.json'
            source = {'name': 'floor', 'texts': [{'text': 'retained'}], 'items': [
                {'file': 'pb/view_live.pb', 'name': 'frame', 'at': [1, 2, 3], 'point_size': 2},
                {'file': 'pb/other.pb', 'encoding': 'gzip', 'size': 123}]}
            manifest.write_text(json.dumps(source))
            output = root / 'prepared.json'
            revision = 'pb/revisions/0123456789abcdef.pb'
            subprocess.run(['python3', str(SCRIPT), str(manifest), str(geometry), revision, str(output), 'gzip'], check=True)
            actual = json.loads(output.read_text())
            expected = json.loads(json.dumps(source))
            expected['items'][0].update(file=revision, encoding='gzip', size=geometry.stat().st_size)
            self.assertEqual(actual, expected)
            subprocess.run(['python3', str(SCRIPT), str(output), str(geometry), revision, str(output)], check=True)
            actual = json.loads(output.read_text())
            self.assertNotIn('encoding', actual['items'][0])
            self.assertNotIn('size', actual['items'][0])
            self.assertEqual(actual['items'][1], source['items'][1])

    def test_rejects_missing_geometry_reference_and_unsupported_encoding(self):
        with tempfile.TemporaryDirectory(dir=WORK) as directory:
            root = Path(directory)
            geometry = root / 'floor.pb'
            geometry.write_bytes(b'bytes')
            manifest = root / 'scene.json'
            manifest.write_text('{"items":[{"file":"other.pb"}]}')
            for encoding in ['', 'brotli']:
                result = subprocess.run(['python3', str(SCRIPT), str(manifest), str(geometry), 'revision.pb', str(root / 'out.json'), encoding], capture_output=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse((root / 'out.json').exists())

if __name__ == '__main__':
    unittest.main()
