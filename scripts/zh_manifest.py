"""Write distribution/latest.json for a herdr-zh release from the built assets.

usage: zh_manifest.py <version> <owner/repo> <tag> <assets-dir> <notes>

install.sh reads this file with line-based awk, so it must stay pretty-printed with the
top-level "assets" and "sha256" blocks ahead of "releases".
"""

import hashlib
import json
import pathlib
import re
import sys

ASSETS = {
    "linux-x86_64": "herdr-linux-x86_64",
    "linux-aarch64": "herdr-linux-aarch64",
    "macos-x86_64": "herdr-macos-x86_64",
    "macos-aarch64": "herdr-macos-aarch64",
    "windows-x86_64": "herdr-windows-x86_64.zip",
}


def source_const(path: str, name: str) -> int:
    text = pathlib.Path(path).read_text(encoding="utf-8")
    match = re.search(rf"pub const {name}: u32 = (\d+);", text)
    assert match, f"{name} not found in {path}"
    return int(match.group(1))


def main() -> None:
    version, repo, tag, assets_dir, notes = sys.argv[1:6]
    assert re.fullmatch(r"\d+\.\d+\.\d+", version), f"version must be X.Y.Z, got {version}"

    base = f"https://github.com/{repo}/releases/download/{tag}/"
    entry = {
        "notes": notes,
        "protocol": source_const("src/protocol/wire.rs", "PROTOCOL_VERSION"),
        "endpoint_generation": source_const(
            "src/protocol/endpoint.rs", "ENDPOINT_PROTOCOL_GENERATION"
        ),
        "assets": {key: base + name for key, name in ASSETS.items()},
        "sha256": {
            key: hashlib.sha256((pathlib.Path(assets_dir) / name).read_bytes()).hexdigest()
            for key, name in ASSETS.items()
        },
    }

    path = pathlib.Path("distribution/latest.json")
    old = json.loads(path.read_text(encoding="utf-8"))
    # The file starts out as upstream's manifest: keep only releases published from this repo.
    ours = {
        ver: release
        for ver, release in old.get("releases", {}).items()
        if ver != version
        and all(url.startswith(f"https://github.com/{repo}/") for url in release["assets"].values())
    }
    manifest = {"version": version, **entry, "releases": {version: entry, **ours}}
    path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {path}: {version}, {len(ours)} earlier release(s) kept")


if __name__ == "__main__":
    main()
