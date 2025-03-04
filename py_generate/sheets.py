from dataclasses import dataclass, field
from py_generate.common import ZF_232, VersionInfo, load_sheet
from py_generate.templates import (
    EnumValue,
    convert_to_identifier,
    generate,
    reset_mod_rs,
)


def run_all(version: VersionInfo = ZF_232):
    reset_mod_rs(version)

    for basic_info in ALL_BASIC:
        run_basic(
            version,
            sheet_name=basic_info.sheet_name,
            code_column=basic_info.code_column,
            description_column=basic_info.name_column,
            extra_columns=basic_info.extra_columns,
            file_name=basic_info.file_name,
            rust_type=basic_info.rust_type,
            write_mod=True,
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
):
    if rust_type is None:
        rust_type = sheet_name
    if file_name is None:
        file_name = f"{rust_type.lower()}.rs"

    sheet = load_sheet(sheet_name, version)

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

    generate(rust_type, enum_values, version, file_name, write_mod)


@dataclass
class BasicInfo:
    sheet_name: str
    code_column: int
    name_column: int
    extra_columns: list[int] = field(default_factory=list)
    file_name: str | None = None
    rust_type: str | None = None


ALL_BASIC = [
    BasicInfo(
        sheet_name="Country",
        code_column=2,
        name_column=1,
    ),
    BasicInfo(
        sheet_name="Currency",
        code_column=2,
        name_column=1,
    ),
    BasicInfo(
        sheet_name="ICD",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: EN16931 Interpretation column
        sheet_name="1001",
        rust_type="Enum1001",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="1153",
        rust_type="Enum1153",
        code_column=1,
        name_column=2,
    ),
    # TODO: VAT ID - do by hand, its 3 things
    # TODO: VAT CAT - more complex table
    BasicInfo(
        sheet_name="Text",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: Usage in EN16931 column
        sheet_name="Payment",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: Semantic model column
        sheet_name="5305",
        rust_type="Enum5305",
        code_column=1,
        name_column=2,
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
    # TODO: MIME is just a list, not a mapping
    BasicInfo(
        sheet_name="EAS",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: Remark column
        sheet_name="VATEX",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: source column
        sheet_name="Unit",
        code_column=2,
        name_column=3,
    ),
    BasicInfo(  # TODO: description column
        sheet_name="Line Status",
        rust_type="LineStatus",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="Language",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: code list column, description column
        sheet_name="Characteristic",
        code_column=2,
        name_column=3,
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
    BasicInfo(  # TODO: description column
        sheet_name="TRANSPORT",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: description column
        sheet_name="Date",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: Usage rule column
        sheet_name="HybridDocument",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(  # TODO: Usage rule column, comment column
        sheet_name="HybridConformance", code_column=1, name_column=2
    ),
    BasicInfo(  # TODO: Usage rule column
        sheet_name="Filename",
        code_column=1,
        name_column=2,
    ),
    BasicInfo(
        sheet_name="HybridVersion",
        code_column=1,
        name_column=2,
        extra_columns=[3],
    ),
]
