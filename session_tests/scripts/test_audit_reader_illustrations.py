import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location('reader_audit', Path(__file__).with_name('audit-reader-illustrations.py'))
reader = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reader)


def svg(identifier, label='Camera', x='3'):
    return f'<svg xmlns="http://www.w3.org/2000/svg"><defs><marker id="{identifier}"><path d="M0 0L1 1"/></marker></defs><path d="M0 0L3 3" marker-end="url(#{identifier})"/><text x="{x}">{label}</text></svg>'


class ReaderAuditTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.docs = Path(self.temporary.name)
        (self.docs / 'journey').mkdir()

    def tearDown(self):
        self.temporary.cleanup()

    def pages(self, chapters):
        book = {'chapters': []}
        for chapter, texts in enumerate(chapters, 1):
            lessons = []
            for index, text in enumerate(texts):
                name = f'journey/{chapter}-{index}.md'
                (self.docs / name).write_text(text, encoding='utf-8')
                lessons.append({'page': name})
            book['chapters'].append({'title': str(chapter), 'consolidated': lessons})
        (self.docs / 'journey/book.json').write_text(json.dumps(book), encoding='utf-8')

    def test_svg_namespace_changes_do_not_hide_cross_chapter_reuse(self):
        self.pages([[svg('chapter-one')], [svg('chapter-two')]])
        result = reader.audit(self.docs)
        self.assertEqual(result['status'], 'FAIL')
        self.assertEqual(result['errors'], [])
        self.assertEqual(result['crossChapterSharedIllustrations'][0]['chapters'], [1, 2])

    def test_visible_labels_and_geometry_are_not_discarded(self):
        self.pages([[svg('one')], [svg('two', label='Clip')], [svg('three', x='4')]])
        self.assertEqual(reader.audit(self.docs)['status'], 'PASS')

    def test_same_chapter_raster_reuse_is_allowed_but_cross_chapter_is_not(self):
        (self.docs / 'one.webp').write_bytes(b'identical image')
        (self.docs / 'two.jpeg').write_bytes(b'identical image')
        self.pages([['![One](../one.webp)', '![Two](../two.jpeg)']])
        self.assertEqual(reader.audit(self.docs)['status'], 'PASS')
        self.pages([['![One](../one.webp)'], ['![Two](../two.jpeg)']])
        self.assertEqual(reader.audit(self.docs)['status'], 'FAIL')

    def test_missing_local_image_fails_and_fenced_example_is_ignored(self):
        self.pages([['```md\n![Example](missing.png)\n```\n' + svg('one')]])
        self.assertEqual(reader.audit(self.docs)['status'], 'PASS')
        self.pages([['![Missing](missing.png)']])
        result = reader.audit(self.docs)
        self.assertEqual(result['status'], 'FAIL')
        self.assertEqual(len(result['errors']), 1)

    def test_linked_svg_and_reference_image_are_resolved(self):
        (self.docs / 'one.svg').write_text(svg('one'), encoding='utf-8')
        (self.docs / 'two.svg').write_text(svg('two'), encoding='utf-8')
        self.pages([['![One][diagram]\n[diagram]: ../one.svg'], ['<img src="../two.svg">']])
        result = reader.audit(self.docs)
        self.assertEqual(result['errors'], [])
        self.assertEqual(len(result['crossChapterSharedIllustrations']), 1)


if __name__ == '__main__':
    unittest.main()
