from __future__ import annotations

# load a .env file
from dataclasses import dataclass
from enum import Enum
from dotenv import load_dotenv
from pathlib import Path

import pandas as pd


load_dotenv()


@dataclass
class VersionInfo:
    version: str
    spec_dir: Path
    src_dir: Path


class Version(str, Enum):
    ZF_232 = "2.3.2"
    ZF_233 = "2.3.3"

    def version_info(self) -> VersionInfo:
        with_underscore = self.value.replace(".", "_")

        if self == Version.ZF_232:
            return VersionInfo(
                version=self.value,
                spec_dir=Path(f"spec/zugferd_{with_underscore}"),
                src_dir=Path(f"src/zugferd_{with_underscore}"),
            )
        elif self == Version.ZF_233:
            return VersionInfo(
                version=self.value,
                spec_dir=Path(f"spec/zugferd_{with_underscore}"),
                src_dir=Path(f"src/zugferd_{with_underscore}"),
            )
        else:
            raise ValueError(f"Unknown version: {self}")


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
