from dataclasses import dataclass, field
import tomllib
from typing import Iterable
from py_generate.common import RS, Code, VersionInfo
from py_generate.find_package_src import get_enum_variants
from py_generate.rendering import TAB, CodeGenerator, EnumValue


@dataclass
class Matches:
    our_enum_unmatched: list[EnumValue] = field(default_factory=list)
    their_enum_unmatched: list[str] = field(default_factory=list)
    matches: list[tuple[str, str]] = field(default_factory=list)


class GenTryFrom(CodeGenerator):

    def __init__(
        self,
        their_enum_rust_qualified_name: str,
        dependency_name: str,
        feature_gate: str | None = None,
        generated_by_build_rs: bool = False,
    ):
        self.__their_enum_rust_qualified_name = their_enum_rust_qualified_name
        self.__feature_gate = feature_gate
        self.__dependency_name = dependency_name
        self.__generated_by_build_rs = generated_by_build_rs

    def generate(
        self,
        enum_name: str,
        enum_values: list[EnumValue],
        version: VersionInfo,
        rs_file: str,
    ) -> Code[str]:
        matches = self.__matcher(enum_values)

        feature_gate = (
            f'#[cfg(feature = "{self.__feature_gate}")]' if self.__feature_gate else ""
        )

        # our enum -> their enum. our enum is the input here.
        # if all of our enum values are matched, then this is a total function,
        # i.e. no error is possible as every input has a valid mapping.
        is_total_func = len(matches.our_enum_unmatched) == 0

        err_from = (
            ""
            if is_total_func
            else f'''
{feature_gate}
/// All the variants of {enum_name} that are not matched to any variant of {self.__their_enum_rust_qualified_name}
pub enum ErrFrom{enum_name} {{
    {'\n'.join(f"{TAB}{TAB}{TAB}{ours.rust_identifier}," for ours in matches.our_enum_unmatched)}
}}
'''
        )

        code_ours_to_theirs = f"""
{feature_gate}
impl std::convert::TryFrom<{enum_name}> for {self.__their_enum_rust_qualified_name} {{
    type Error = {'std::convert::Infallible' if is_total_func else f'ErrFrom{enum_name}'};
    fn try_from(value: {enum_name}) -> Result<Self, Self::Error> {{
        match value {{
            {'\n'.join(f"{TAB}{TAB}{TAB}{enum_name}::{ours} => Ok({self.__their_enum_rust_qualified_name}::{theirs})," for (ours, theirs) in matches.matches)}
            {'\n'.join(f"{TAB}{TAB}{TAB}{enum_name}::{ours.rust_identifier} => Err(ErrFrom{enum_name}::{ours.rust_identifier})," for ours in matches.our_enum_unmatched)}
        }}
    }}
}}

{err_from}
"""

        is_total_func = len(matches.their_enum_unmatched) == 0

        their_enum_as_ident = "".join(
            word.capitalize()
            for ident in self.__their_enum_rust_qualified_name.split("::")
            for word in ident.split("_")
        )

        err_from = (
            ""
            if is_total_func
            else f'''
{feature_gate}
/// All the variants of {self.__their_enum_rust_qualified_name} that are not matched to any variant of {enum_name}
pub enum ErrFrom{their_enum_as_ident} {{
    {'\n'.join(f"{TAB}{TAB}{TAB}{theirs}," for theirs in matches.their_enum_unmatched)}
}}
'''
        )

        code_theirs_to_ours = f"""
{feature_gate}
impl std::convert::TryFrom<{self.__their_enum_rust_qualified_name}> for {enum_name} {{
    type Error = {'std::convert::Infallible' if is_total_func else f'ErrFrom{their_enum_as_ident}'};
    fn try_from(value: {self.__their_enum_rust_qualified_name}) -> Result<{enum_name}, Self::Error> {{
        match value {{
            {'\n'.join(f"{TAB}{TAB}{TAB}{self.__their_enum_rust_qualified_name}::{theirs} => Ok({enum_name}::{ours})," for (ours, theirs) in matches.matches)}
            {'\n'.join(f"{TAB}{TAB}{TAB}{self.__their_enum_rust_qualified_name}::{theirs} => Err(ErrFrom{their_enum_as_ident}::{theirs})," for theirs in matches.their_enum_unmatched)}
        }}
    }}
}}

{err_from}
"""

        return RS(code_ours_to_theirs + code_theirs_to_ours)

    def __get_variants(self) -> Iterable[str]:
        with open("Cargo.toml", "rb") as f:
            cargo_toml = tomllib.load(f)
        version = cargo_toml["dependencies"][self.__dependency_name]["version"]

        enum_name = self.__their_enum_rust_qualified_name.split("::")[-1]

        return (
            v.decode("utf-8")
            for v in get_enum_variants(
                self.__dependency_name,
                version,
                enum_name.encode(),
                self.__generated_by_build_rs,
            )
        )

    def __matcher(self, enum_values: list[EnumValue]) -> Matches:
        # key is just for case-insensitive matching
        variants = {v.upper(): v for v in self.__get_variants()}

        matches = Matches()
        for enum_value in enum_values:
            if enum_value.code.upper() in variants:
                matches.matches.append(
                    (enum_value.rust_identifier, variants[enum_value.code.upper()])
                )
                del variants[enum_value.code.upper()]
            else:
                matches.our_enum_unmatched.append(enum_value)

        for variant in variants.values():
            matches.their_enum_unmatched.append(variant)

        return matches


ISO_COUNTRY_TRY_FROM = GenTryFrom(
    "iso_country::Country", feature_gate="iso_country", dependency_name="iso_country"
)
ISO_CURRENCY_TRY_FROM = GenTryFrom(
    "iso_currency::Currency",
    feature_gate="iso_currency",
    dependency_name="iso_currency",
    generated_by_build_rs=True,
)
