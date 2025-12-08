from collections import defaultdict
from dataclasses import dataclass
from hashlib import md5
from pathlib import Path
import re
from typing import Literal

from py_generate.common import Version


def group_by_shared_content(files: list[Path]) -> list[list[Path]]:
    # for the purpose of comparing, remove code between // Start: (Version) TryFrom and // End: (Version) TryFrom

    def reader(p: Path) -> str:
        with p.open() as f:
            content = f.read()

        # ignoring the TryFrom versions is ok since after deduplication,
        # the enum will be the same and rust provides a blanket try_from impl
        # for a type to itself
        m = re.search(
            r"^// Start: \(Version\) TryFrom.*?// End: \(Version\) TryFrom[^\n]*$",
            content,
            flags=re.DOTALL | re.MULTILINE,
        )
        if m is not None:
            # cut out the code between the start and end markers
            content = content[: m.start()] + content[m.end() :]
        elif "mod.rs" in str(p) or "zugferd_2_3_2" in str(p) or "lib.rs" in str(p):
            pass
        else:
            # Could be concerning, but usually fine
            print(f"No start and end markers found in {p}")

        # standardize whitespace
        content = re.sub(r"\t", " ", content).strip()
        content = re.sub(r"[ ]{2,}", " ", content).strip()

        return content

    # hash each file and group by the hash
    by_hash = defaultdict(list)
    for file in files:
        content = reader(file)
        hash = md5(content.encode()).hexdigest()
        by_hash[hash].append(file)

    true_repeated_files = []
    for hash, files in by_hash.items():
        if len(files) <= 1:
            continue

        # ensure the files are truly the same
        content, *contents = [reader(file) for file in files]
        if not all(content == c for c in contents):
            raise ValueError(f"md5 collision: Files {files} are not the same")

        true_repeated_files.append(files)

    return true_repeated_files


def get_version(file: Path, relative_to: Path) -> Version:
    VERSION_LOOKUP = {version.version_info().src_dir: version for version in Version}

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
