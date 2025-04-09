# load a .env file
from dataclasses import dataclass
from os import getenv
from dotenv import load_dotenv
from pathlib import Path

import pandas as pd

load_dotenv()


def _get_path(var_name: str) -> Path:
    value = getenv(var_name)
    if value is None:
        raise ValueError(f"Environment variable {var_name} is not set")
    return Path(value.strip())


@dataclass
class VersionInfo:
    version: str
    spec_dir: Path
    src_dir: Path


ZF_232 = VersionInfo(
    version="2.3.2",
    spec_dir=Path("spec/zugferd_2_3_2"),
    src_dir=Path("src/zugferd_2_3_2"),
)

TS_DIR = Path("ts")

EXCEL_FILE = "4. EN16931+FacturX code lists values v14 - used from 2024-11-15.xlsx"


def load_all_sheets(version: VersionInfo) -> dict[str, pd.DataFrame]:
    file_path = version.spec_dir / EXCEL_FILE
    if not file_path.exists():
        raise FileNotFoundError(f"File {file_path} not found")
    all_sheets = pd.read_excel(file_path, sheet_name=None)

    return all_sheets


def load_sheet(sheet: str, version: VersionInfo, header_idx: int) -> pd.DataFrame:
    file_path = version.spec_dir / EXCEL_FILE
    if not file_path.exists():
        raise FileNotFoundError(f"File {file_path} not found")
    return pd.read_excel(file_path, sheet_name=sheet, dtype=str, header=header_idx)


@dataclass
class RS_TS[T]:
    rs: T
    ts: T
