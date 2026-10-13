import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("release_version", Path(__file__).with_name("release-version.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ReleaseVersionTests(unittest.TestCase):
    manifest = {"workspace": {"package": {"version": "0.7.1"}}}

    def test_default_and_override(self):
        self.assertEqual(module.resolve("", self.manifest), "0.7.1")
        self.assertEqual(module.resolve("0.8.0-rc.1", self.manifest), "0.8.0-rc.1")

    def test_rejects_shell_and_output_injection(self):
        for value in ("$(id)", "0.7.1\nversion=9.9.9", "0.7.1; id", "../0.7.1", "01.7.1", "0.7.1-", " 0.7.1"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                module.resolve(value, self.manifest)

    def test_published_releases_cannot_be_changed(self):
        release_spec = importlib.util.spec_from_file_location("draft_release", Path(__file__).with_name("draft-release.py"))
        release = importlib.util.module_from_spec(release_spec)
        release_spec.loader.exec_module(release)
        release.require_draft({"isDraft": True})
        for info in ({"isDraft": False}, {}):
            with self.subTest(info=info), self.assertRaises(ValueError):
                release.require_draft(info)


if __name__ == "__main__":
    unittest.main()
