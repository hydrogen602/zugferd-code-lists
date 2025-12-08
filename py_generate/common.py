from __future__ import annotations

# load a .env file
from dataclasses import dataclass
from enum import Enum
from functools import total_ordering
import itertools
from dotenv import load_dotenv
from pathlib import Path

import pandas as pd


load_dotenv()


@dataclass
class VersionInfo:
    version: Version
    spec_dir: Path
    src_dir: Path


@total_ordering
class Version(str, Enum):
    ZF_232 = "2.3.2"
    ZF_233 = "2.3.3"
    ZF_24 = "2.4"

    def version_info(self) -> VersionInfo:

        return VersionInfo(
            version=self,
            spec_dir=Path(f"spec/{self.version_folder}"),
            src_dir=Path(f"src/{self.version_folder}"),
        )

    @property
    def version_folder(self) -> str:
        return f"zugferd_{self.value.replace('.', '_')}"

    def __lt__(self, other: Version | str):
        if not isinstance(other, Version):
            return NotImplemented

        self_parts = self.value.split(".")
        other_parts = other.value.split(".")

        for self_part_, other_part_ in itertools.zip_longest(
            self_parts, other_parts, fillvalue="0"
        ):
            self_part = int(self_part_)
            other_part = int(other_part_)
            if self_part < other_part:
                return True
            elif self_part > other_part:
                return False
            else:
                continue

        return False  # they are the same


TS_DIR = Path("ts")


def _find_excel_file(version: VersionInfo) -> Path:
    files = list(version.spec_dir.glob("*.xlsx"))
    if len(files) == 0:
        raise FileNotFoundError(f"No Excel files found in {version.spec_dir}")
    elif len(files) > 1:
        raise FileNotFoundError(
            f"Ambiguous: multiple excel files found in {version.spec_dir}"
        )
    return files[0]


def load_all_sheets(version: VersionInfo) -> dict[str, pd.DataFrame]:
    file_path = _find_excel_file(version)
    return pd.read_excel(file_path, sheet_name=None, keep_default_na=False)


def load_sheet(sheet: str, version: VersionInfo, header_idx: int = 0) -> pd.DataFrame:
    file_path = _find_excel_file(version)
    return pd.read_excel(
        file_path, sheet_name=sheet, dtype=str, header=header_idx, keep_default_na=False
    )


@dataclass
class RS_TS[T]:
    rs: T
    ts: T

    @classmethod
    def combine(cls, rs: T, ts: T) -> RS_TS[T]:
        return cls(rs=rs, ts=ts)


@dataclass
class RS[T]:
    rs: T


@dataclass
class TS[T]:
    ts: T


type Code[T] = RS[T] | TS[T] | RS_TS[T]
