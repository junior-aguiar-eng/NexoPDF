"""Create/update a draft from built packages; never modify a published release."""
from pathlib import Path
import hashlib
import json
import os
import subprocess


def require_draft(info):
    if not info.get("isDraft"):
        raise ValueError("This version is already published; choose a new version")


def digest(path):
    with path.open("rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()


if __name__ == "__main__":
    version = os.environ["NEXOPDF_VERSION"]
    commit = os.environ["GITHUB_SHA"]
    tag = "v" + version
    dist = Path(__file__).resolve().parents[1] / "dist/release"
    files = sorted(p for p in dist.iterdir() if p.is_file() and p.name != "SHA256SUMS.txt")
    if not files:
        raise ValueError("No release packages were built")
    checksum = dist / "SHA256SUMS.txt"
    checksum.write_text("".join(f"{digest(p)}  {p.name}\n" for p in files), encoding="utf-8")
    existing = subprocess.run(["gh", "release", "view", tag, "--json", "isDraft"], capture_output=True, text=True)
    if existing.returncode == 0:
        require_draft(json.loads(existing.stdout))
        subprocess.run(["gh", "release", "edit", tag, "--target", commit], check=True)
    else:
        args = ["gh", "release", "create", tag, "--draft", "--target", commit, "--title", "NexoPDF v" + version, "--generate-notes"]
        if "-" in version:
            args.append("--prerelease")
        subprocess.run(args, check=True)
    subprocess.run(["gh", "release", "upload", tag, "--clobber", *map(str, files), str(checksum)], check=True)
