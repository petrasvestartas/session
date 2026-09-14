import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


class FormatterArgumentsTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.script = self.root / "bash" / "format.py"
        self.script.parent.mkdir()
        shutil.copyfile(Path(__file__).with_name("format.py"), self.script)
        self.sources = []
        self.originals = []
        for name in ("point_test.cpp", "other_test.cpp"):
            source = self.root / "session_cpp" / "src" / name
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_text("auto point = Point(0,0,0);\n")
            self.sources.append(source)
            self.originals.append(source.read_bytes())

    def run_formatter(self, *args):
        return subprocess.run(
            [sys.executable, str(self.script), *args],
            cwd=self.root,
            capture_output=True,
            text=True,
            timeout=10,
        )

    def assert_sources_unchanged(self):
        for index in range(len(self.sources)):
            self.assertEqual(self.sources[index].read_bytes(), self.originals[index])

    def test_help_never_formats(self):
        for flag in ("--help", "-h"):
            with self.subTest(flag=flag):
                result = self.run_formatter(flag)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn("Usage:", result.stdout)
                self.assert_sources_unchanged()

    def test_invalid_arguments_never_format(self):
        for args in (("--unknown",), ("missing.cpp",), ("--cpp", "--unknown")):
            with self.subTest(args=args):
                result = self.run_formatter(*args)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("Unknown option or missing file", result.stderr)
                self.assert_sources_unchanged()

    def test_dry_run_never_formats(self):
        result = self.run_formatter("--dry-run")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("WOULD CHANGE", result.stdout)
        self.assert_sources_unchanged()

    def test_explicit_file_keeps_other_files_unchanged(self):
        result = self.run_formatter(str(self.sources[0]))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.sources[0].read_text(), "auto point = Point(0, 0, 0);\n")
        self.assertEqual(self.sources[1].read_bytes(), self.originals[1])

    def test_default_still_formats_all_sources(self):
        result = self.run_formatter()
        self.assertEqual(result.returncode, 0, result.stderr)
        for source in self.sources:
            self.assertEqual(source.read_text(), "auto point = Point(0, 0, 0);\n")

    def test_strings_and_comments_keep_their_contents(self):
        fixtures = {
            "cpp": (
                'auto text = "NurbsSurface(degree=(3,3), cvs=(4,4))";\n'
                'auto raw = R"cad(Point(1,2,3) " {1,2,3}\nPoint(4,5,6))cad";\n'
                'auto escaped = "Point(1,2,3) \\"quoted\\"";\n'
                '// Point(1,2,3), Point(4,5,6)\n'
                '// Continued comment \\\nPoint(1,2,3), Point(4,5,6)\n'
                '/* Point(1,2,3)\nPoint(4,5,6) */\n'
                "auto count = 1'000;\n"
            ),
            "py": (
                'text = "Point(1,2,3)"\n'
                'raw = r"Point(1,2,3)"\n'
                'doc = """Point(1,2,3)\nPoint(4,5,6)"""\n'
                '# Point(1,2,3), Point(4,5,6)\n'
            ),
            "rs": (
                'let text = "Point::new(1,2,3)";\n'
                'let raw = r##"Point::new(1,2,3) "#\nPoint::new(4,5,6)"##;\n'
                '/* Point::new(1,2,3) /* nested */ Point::new(4,5,6) */\n'
                "fn borrow<'a>(v: &'a str) -> &'a str { v }\n"
            ),
        }
        for ext, protected in fixtures.items():
            with self.subTest(language=ext):
                constructor = 'Point::new(0,0,0)' if ext == 'rs' else 'Point(0,0,0)'
                source = self.root / f'literals.{ext}'
                source.write_text(protected + constructor + '\n')
                result = self.run_formatter(str(source))
                self.assertEqual(result.returncode, 0, result.stderr)
                expected = protected + constructor.replace(',', ', ') + '\n'
                self.assertEqual(source.read_text(), expected)
                result = self.run_formatter(str(source))
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(source.read_text(), expected)

    def test_constructor_string_arguments_are_preserved(self):
        source = self.sources[0]
        source.write_text('auto p = Point("x,y",0,0); // Point(1,2,3)\n')
        result = self.run_formatter(str(source))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(source.read_text(), 'auto p = Point("x,y", 0, 0); // Point(1,2,3)\n')

    def test_collection_comments_do_not_swallow_following_code(self):
        for comment in ('// Keep this point', '/* Keep this point */'):
            with self.subTest(comment=comment):
                source = self.sources[0]
                original = (
                    'auto p = Polyline({Point(0,0,0),\n'
                    f'    Point(1,1,1), {comment}\n'
                    '    Point(2,2,2)});\n'
                )
                source.write_text(original)
                result = self.run_formatter(str(source))
                self.assertEqual(result.returncode, 0, result.stderr)
                expected = original.replace('(0,0,0)', '(0, 0, 0)').replace('(1,1,1)', '(1, 1, 1)').replace('(2,2,2)', '(2, 2, 2)')
                self.assertEqual(source.read_text(), expected)


if __name__ == "__main__":
    unittest.main()
