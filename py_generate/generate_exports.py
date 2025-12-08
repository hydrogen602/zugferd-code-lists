#!/usr/bin/env python3
"""Generate exports dict for package.json based on ts/src structure."""

from __future__ import annotations

import json

from pathlib import Path
from typing import TypedDict

ExportDef = TypedDict(
    "ExportDef",
    {
        "types": str,
        "import": str,
        "require": str,
    },
)


def generate_exports(src_dir: Path) -> dict[str, ExportDef]:

    assert src_dir.exists(), f"Source directory {src_dir} does not exist"

    exports: dict[str, ExportDef] = {}

    # recursively find all subdirectories with any .ts file
    for item in src_dir.rglob("*.ts"):
        if item.is_file():
            rel_path = item.relative_to(src_dir)
            ts_name = rel_path.with_suffix("")
            if ts_name.stem == "index":
                ts_name = ts_name.parent
            name = "./" + str(ts_name)
            if name == "./.":
                name = "."  # special case for root
            exports[name] = {
                "types": f"./dist/{rel_path.with_suffix('.d.ts')}",
                "import": f"./dist/{rel_path.with_suffix('.mjs')}",
                "require": f"./dist/{rel_path.with_suffix('.js')}",
            }
    return exports


def update_package_json(package_json_path: Path, src_dir: Path) -> None:
    """Update package.json with generated exports dict."""
    # Read current package.json
    with open(package_json_path, "r", encoding="utf-8") as f:
        package_data = json.load(f)

    # Generate exports
    exports_dict = generate_exports(src_dir)

    # Update exports field
    package_data["exports"] = exports_dict

    # Write back to package.json
    with open(package_json_path, "w", encoding="utf-8") as f:
        json.dump(package_data, f, indent=2)
        f.write("\n")

    print(f"Updated {package_json_path}")
    print(f"Generated {len(exports_dict)} export entries")


def main() -> None:
    """Main entry point."""
    script_dir = Path(__file__).parent
    ts_dir = script_dir / "ts"
    src_dir = ts_dir / "src"
    package_json_path = ts_dir / "package.json"

    if not package_json_path.exists():
        print(f"Error: {package_json_path} not found")
        return

    update_package_json(package_json_path, src_dir)
