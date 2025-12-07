from collections import defaultdict
from contextlib import ExitStack
from dataclasses import dataclass
from hashlib import md5
from pathlib import Path
import re
from typing import Literal

from py_generate.common import Version

VERSION_LOOKUP = {version.version_info().src_dir: version for version in Version}


def group_by_shared_content(files: list[Path]) -> list[list[Path]]:
    # hash each file and group by the hash
    by_hash = defaultdict(list)
    for file in files:
        with open(file, "rb") as f:
            hash = md5(f.read()).hexdigest()
            by_hash[hash].append(file)

    true_repeated_files = []
    for hash, files in by_hash.items():
        if len(files) <= 1:
            continue

        # ensure the files are truly the same
        with ExitStack() as stack:
            f_handles = [stack.enter_context(open(file, "rb")) for file in files]

            while True:
                common_chunk = None
                for f_handle in f_handles:
                    chunk = f_handle.read(1024)
                    if common_chunk is None:
                        common_chunk = chunk
                    elif chunk != common_chunk:
                        raise ValueError(
                            f"md5 collision: Files {files} are not the same"
                        )

                assert common_chunk is not None
                if len(common_chunk) == 0:
                    break

        true_repeated_files.append(files)

    return true_repeated_files


def get_version(file: Path, relative_to: Path) -> Version:
    dir = file.relative_to(relative_to).parent  # remove the file part
    return VERSION_LOOKUP[dir]


def get_enum_name(file: Path, lang: Literal["ts", "rs"]) -> str:
    match lang:
        case "ts":
            m = re.search(r"export enum (\w+)", file.read_text())
        case "rs":
            m = re.search(r"pub enum (\w+)", file.read_text())

    assert m is not None, f"Expected enum name in {file}"
    return m.group(1)


def deduplicate_files(files: list[Path], folders: Folders):
    assert len(files) > 1, f"Expected at least two files, got {len(files)}"
    # pick the oldest version as the canonical version
    files.sort(key=lambda x: get_version(x, relative_to=folders.project))
    canonical_file, *other_files = files

    enum_name = get_enum_name(canonical_file, folders.lang)

    for other_file in other_files:
        print("Deduplicating", other_file)
        with other_file.open("w") as f:
            canonical_file_relative = canonical_file.relative_to(
                folders.src
            ).with_suffix("")
            if folders.lang == "ts":
                f.write(
                    f'import {{ {enum_name}, description}} from "../{canonical_file_relative}"\n'
                )
                f.write(f"export {{ {enum_name}, description }}")
            elif folders.lang == "rs":
                path = "::".join(canonical_file_relative.parts)
                f.write("#![allow(non_camel_case_types)]\n")
                f.write(f"pub use crate::{path}::{enum_name};\n")
            else:
                raise ValueError(f"Invalid language: {folders.lang}")


@dataclass
class Folders:
    project: Path
    src: Path
    lang: Literal["ts", "rs"]


def get_folders(lang: Literal["ts", "rs"]) -> Folders:
    match lang:
        case "ts":
            return Folders(project=Path("ts"), src=Path("ts/src"), lang="ts")
        case "rs":
            return Folders(project=Path(), src=Path("src"), lang="rs")


def main(lang: Literal["ts", "rs"]):
    # search ts/src for repeated code (two files with the same content)
    # as some code lists are unchanged across multiple versions
    # the files would have the same name too

    folders = get_folders(lang)

    by_stem = defaultdict(list)
    for file in folders.src.rglob(f"*.{lang}"):
        by_stem[file.stem].append(file)

    true_repeated_files = []
    for files in by_stem.values():
        true_repeated_files.extend(group_by_shared_content(files))

    for files in true_repeated_files:
        deduplicate_files(files, folders=folders)
