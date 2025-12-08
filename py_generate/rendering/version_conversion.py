from typing import Self, TYPE_CHECKING

from py_generate.rendering.gen_try_from import GenTryFrom, TheirEnumValue

if TYPE_CHECKING:
    from py_generate.common import Code, VersionInfo, Version
    from py_generate.sheets import GenerationInfo
    from py_generate.rendering import EnumValue


class ConvertFrom(GenTryFrom):
    """
    For conversions from one version to another, e.g. Allowance 2.3.2 to Allowance 2.3.3.
    """

    def __init__(
        self,
        from_qualified_enum_name: str,
        from_enum_values: list[EnumValue],
        from_version: Version,
    ):
        self.__from_version = from_version
        super().__init__(
            from_qualified_enum_name=from_qualified_enum_name,
            from_enum_values=[
                TheirEnumValue.from_enum_value(e) for e in from_enum_values
            ],
            feature_gate=None,
            conversion_name="Version",
        )

    @classmethod
    def from_generation_info(cls, generation_info: GenerationInfo) -> Self:
        return cls(
            from_qualified_enum_name=f"crate::{generation_info.version.version.version_folder}::{generation_info.enum_name}",
            from_enum_values=generation_info.enum_values,
            from_version=generation_info.version.version,
        )

    def generate(
        self,
        enum_name: str,
        enum_values: list[EnumValue],
        version: VersionInfo,
        rs_file: str,
    ) -> Code[str]:
        if version.version == self.__from_version:
            raise ValueError(
                f"Version {version.version} is the same as the from version {self.__from_version}"
            )
        if enum_name != self.from_enum_name:
            raise ValueError(
                f"Enum name {enum_name} is not the same as the from enum name {self.from_enum_name}"
            )

        return super().generate(enum_name, enum_values, version, rs_file)
