from dataclasses import dataclass, field
from typing import Iterable
from rich.progress import track

from py_generate.common import Version, VersionInfo, load_sheet
from py_generate.rendering import CodeGenerator
from py_generate.rendering.gen_try_from import (
    ISO_COUNTRY_TRY_FROM,
    ISO_CURRENCY_TRY_FROM,
)
from py_generate.rendering.templates import (
    EnumValue,
    convert_to_identifier,
    generate,
    reset_mod_rs,
)


def run_all(version: VersionInfo):
    reset_mod_rs(version)

    for basic_info in track(ALL_BASIC, description=f"Generating v{version.version}"):
        if (
            basic_info.version_filter is not None
            and version.version not in basic_info.version_filter
        ):
            continue

        run_basic(
            version,
            sheet_name=basic_info.sheet_name,
            code_column=basic_info.code_column,
            description_column=basic_info.name_column,
            extra_columns=basic_info.extra_columns,
            file_name=basic_info.file_name,
            rust_type=basic_info.rust_type,
            write_mod=True,
            header_idx=basic_info.header_index,
            extra_generators=basic_info.extra_generators,
        )


def run_basic(
    version: VersionInfo,
    sheet_name: str,
    code_column: int,
    description_column: int,
    extra_columns: list[int] | None = None,
    file_name: str | None = None,
    rust_type: str | None = None,
    write_mod: bool = False,
    header_idx: int = 0,
    extra_generators: Iterable[CodeGenerator] = (),
):
    if rust_type is None:
        rust_type = sheet_name
    if file_name is None:
        file_name = f"{rust_type.lower()}.rs"

    sheet = load_sheet(sheet_name, version, header_idx)

    all_ids = set()

    enum_values = []

    for row in sheet.itertuples(index=True):
        identifier = convert_to_identifier(row[description_column])

        extras = [str(row[i]) for i in extra_columns] if extra_columns else None

        while identifier in all_ids:
            identifier = f"{identifier}_Dup"

        e = EnumValue(
            rust_identifier=identifier,
            description=row[description_column],
            code=row[code_column],
            extras=extras,
        )

        all_ids.add(e.rust_identifier)
        enum_values.append(e)

    generate(
        rust_type,
        enum_values,
        version,
        file_name,
        write_mod,
        extra_generators=extra_generators,
    )


@dataclass
class BasicInfo:
    sheet_name: str
    code_column: int
    name_column: int
    extra_columns: list[int] = field(default_factory=list)
    file_name: str | None = None
    rust_type: str | None = None
    header_index: int = 0
    version_filter: list[Version] | None = None
    extra_generators: list[CodeGenerator] = field(default_factory=list)


ALL_BASIC = [
    BasicInfo(
        sheet_name="Country",
        code_column=2,
        name_column=1,
        extra_generators=[ISO_COUNTRY_TRY_FROM],
    ),
    BasicInfo(
        sheet_name="Currency",
        code_column=2,
        name_column=1,
        extra_generators=[ISO_CURRENCY_TRY_FROM],
    ),
    BasicInfo(
        sheet_name="ICD",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: EN16931 Interpretation column - enum?
        sheet_name="1001",
        rust_type="Enum1001",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="1153",
        rust_type="Enum1153",
        code_column=1,
        name_column=2,
    ),
    # TODO: VAT ID - seems like a mapping between UBL and CII?
    BasicInfo(
        sheet_name="FISCAL ID",
        code_column=1,
        name_column=2,
        version_filter=[Version.ZF_233],  # This sheet doesn't exist in 2.3.2
        rust_type="FiscalID",
    ),
    BasicInfo(
        sheet_name="VAT CAT",
        code_column=3,
        name_column=4,
        extra_columns=[5],
        header_index=5,
        rust_type="VATCAT",
    ),
    BasicInfo(
        sheet_name="Text",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="Payment",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="5305",
        rust_type="Enum5305",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="Allowance",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="Item",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="Charge",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="MIME",
        code_column=1,
        name_column=1,
    ),
    BasicInfo(
        sheet_name="EAS",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="VATEX",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="Unit",
        code_column=2,
        name_column=3,
        extra_columns=[1, 4],
    ),
    BasicInfo(  # TODO: description column
        sheet_name="Line Status",
        rust_type="LineStatus",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="Language",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: code list column - enum?
        sheet_name="Characteristic",
        code_column=2,
        name_column=3,
        extra_columns=[1, 4],
    ),
    BasicInfo(
        sheet_name="Line Reason",
        rust_type="LineReason",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="INCOTERMS",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="TRANSPORT",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="Date",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(  # TODO: Usage rule column - enum?
        sheet_name="HybridDocument",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(  # TODO: Usage rule column - enum?
        sheet_name="HybridConformance",
        code_column=1,
        name_column=2,
        extra_columns=[3, 4],
    ),
    BasicInfo(  # TODO: Usage rule column - enum?
        sheet_name="Filename",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="HybridVersion",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
    BasicInfo(
        sheet_name="Time",
        rust_type="TimeCII",
        code_column=3,
        name_column=4,
        header_index=1,
    ),
]
