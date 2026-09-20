"""Regression checks for the shared numbered/current lesson renderer and auditor."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("course_pages", Path(__file__).resolve().parents[1] / "course_pages.py")
COURSE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COURSE)

PATCH = """diff --git a/src/example.rs b/src/example.rs
--- a/src/example.rs
+++ b/src/example.rs
@@ -1,2 +1,2 @@
-const COUNT: usize = 1;
+const COUNT: usize = 2;
 fn main() {}
"""


class CurrentCourseTests(unittest.TestCase):
    def step(self):
        changes = COURSE.parse_patch(PATCH, "session_viewer")
        change = changes["session_viewer/src/example.rs"]
        change.old_text = "const COUNT: usize = 1;\nfn main() {}\n"
        change.new_text = "const COUNT: usize = 2;\nfn main() {}\n"
        return COURSE.Step({"id": "current-10"}, changes, {}, {})

    def test_prefixed_patch_replays_a_current_lesson_hunk(self):
        step = self.step()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "session_viewer/src/example.rs"
            source.parent.mkdir(parents=True)
            source.write_text(step.changes["session_viewer/src/example.rs"].old_text)
            text = "<!-- file: current-10 session_viewer/src/example.rs type hunks=1 -->"
            result = COURSE.replay_lesson(step, text, root)
        self.assertEqual(result["session_viewer/src/example.rs"], step.changes["session_viewer/src/example.rs"].new_text)

    def test_missing_or_duplicate_hunk_does_not_reproduce_the_checkpoint(self):
        step = self.step()
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "session_viewer/src/example.rs"
            source.parent.mkdir(parents=True)
            source.write_text(step.changes["session_viewer/src/example.rs"].old_text)
            missing = COURSE.replay_lesson(step, "", root)
            self.assertNotEqual(missing["session_viewer/src/example.rs"], step.changes["session_viewer/src/example.rs"].new_text)
            directive = "<!-- file: current-10 session_viewer/src/example.rs type -->"
            with self.assertRaises(ValueError):
                COURSE.replay_lesson(step, directive + "\n" + directive, root)

    def test_unknown_lesson_is_an_audit_error(self):
        step = self.step()
        with patch.object(COURSE, "snapshot_steps", return_value=({}, [step])):
            errors = COURSE.audit(Path("unused"), {}, ["current-99"])
        self.assertEqual(errors, ["unknown lesson ids: current-99"])

    def test_compact_prose_stays_separate_from_the_rendered_file_block(self):
        text = "Keep the count current.\n<!-- file: current-10 session_viewer/src/example.rs type -->"
        rendered = COURSE.Renderer([self.step()], "lessons", {}).expand(text)
        self.assertIn("Keep the count current.\n\n**File:**", rendered)
        self.assertIn("lessons/current-10/raw/session_viewer--src--example.rs.txt", rendered)

    def test_existing_options_and_numbered_ids_still_parse(self):
        text = "<!-- file: 04a session_viewer/src/example.rs copy whole lines=1-2 -->\n<!-- checkpoint: current-11 -->"
        matches = list(COURSE.DIRECTIVE.finditer(text))
        self.assertEqual([m.group(2) for m in matches], ["04a", "current-11"])
        options = COURSE.parse_args_text(matches[0].group(3))
        self.assertEqual((options["label"], options["whole"], options["lines"]), ("copy", True, (1, 2)))

    def test_current_source_links_and_lockfile_policy(self):
        step = self.step()
        lock = COURSE.Change("session_viewer/Cargo.lock")
        lock.new_text = "# Cargo owns this file\n"
        step.changes[lock.name] = lock
        step.texts = dict.fromkeys(step.changes)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            COURSE.write_downloads([step], root)
            COURSE.write_source_index([step], root)
            index = (root / "current-10/index.md").read_text()
            self.assertIn("session_viewer--src--example.rs.txt", index)
            self.assertNotIn("Cargo.lock", index)
            self.assertFalse((root / "current-10/raw/session_viewer--Cargo.lock.txt").exists())

    def test_current_lessons_follow_numbered_checkpoints_numerically(self):
        series = COURSE.REPLAY.read_series()
        selected = COURSE.REPLAY.selected_steps(series, "current-10")
        self.assertEqual([s["id"] for s in selected[-11:]], ["21"] + [f"current-{i}" for i in range(1, 11)])
        self.assertEqual(COURSE.lesson_for("current-1").name, "current-1.md")


if __name__ == "__main__":
    unittest.main()
