"""Local packaging contracts; run with Python 3.11+ and PyYAML."""
from pathlib import Path
import hashlib
import tomllib
import xml.etree.ElementTree as ET
import yaml


def validate(root):
    workflows = root / ".github/workflows"
    ci = yaml.safe_load((workflows / "ci.yml").read_text())
    tests = [step for step in ci["jobs"]["test"]["steps"] if step.get("name") == "Test"]
    assert len(tests) == 1 and "if" not in tests[0], "Workspace tests must run on Linux and Windows with local fonts"
    release = yaml.safe_load((workflows / "release.yml").read_text())
    for name in ("windows", "macos", "draft"):
        assert release["jobs"][name].get("if") == "github.ref == 'refs/heads/release'", f"Release branch guard missing: {name}"
        assert "environment" not in release["jobs"][name], "Do not implicitly create a GitHub environment"
    for path in workflows.glob("*.yml"):
        data = yaml.safe_load(path.read_text())
        # BaseLoader preserves GitHub's literal 'on' key instead of YAML 1.1 booleans.
        assert "on" in yaml.load(path.read_text(), Loader=yaml.BaseLoader), f"Missing workflow triggers: {path}"
        assert "storytold/" not in path.read_text(), f"External project-specific workflow: {path}"
        assert "CRAFT_FONTS_DIR" not in path.read_text(), f"External font checkout: {path}"
        for job in data["jobs"].values():
            assert "uses" not in job, f"Jobs must be defined inside NexoPDF: {path}"
            for step in job.get("steps", []):
                uses = step.get("uses", "")
                if uses and not uses.startswith("./"):
                    assert "@" in uses, f"Unversioned action: {uses}"
    desktop = root / "packaging/linux/io.nexohub.nexopdf.desktop"
    assert b"\r" not in desktop.read_bytes(), "Desktop metadata must use LF"
    for path in (root / "packaging").rglob("*.wxs"):
        ET.parse(path)
    src = yaml.safe_load((root / "packaging/linux/flatpak/io.nexohub.nexopdf.yml").read_text())
    bundle = yaml.safe_load((root / "packaging/linux/flatpak/io.nexohub.nexopdf.bundle.yml").read_text())
    for key in ("runtime", "runtime-version", "sdk", "finish-args"):
        assert src[key] == bundle[key], f"Flatpak manifests disagree on {key}"
    manifest = tomllib.loads((root / "ATTRIBUTION.toml").read_text())
    assets = {item["path"]: item for item in manifest["asset"]}
    font_root = root / "assets/fonts/international"
    count = 0
    for line in (font_root / "manifest.txt").read_text().splitlines():
        if not line or line.startswith("#"):
            continue
        family, style, file, scripts, licence, notice, expected, source = line.split(" | ")
        assert not family.startswith(("Source Han", "Source Serif", "Source Sans", "Noto Sans CJK", "Noto Serif CJK")), family
        assert licence == "OFL-1.1", family
        path = font_root / file
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected, file
        assert (font_root / notice).is_file(), notice
        assert assets[path.relative_to(root).as_posix()]["sha256"] == expected, file
        count += 1
    assert count == 6, f"Expected six local international faces, found {count}"
    print("Local workflows, packaging metadata and six attributed font faces verified")


if __name__ == "__main__":
    validate(Path(__file__).resolve().parents[1])
