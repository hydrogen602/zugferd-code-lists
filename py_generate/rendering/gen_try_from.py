from dataclasses import dataclass, field
import tomllib
from typing import Iterable
from py_generate.common import RS, Code, VersionInfo
from py_generate.find_package_src import get_enum_variants
from py_generate.rendering import CodeGenerator, EnumValue


@dataclass
class Matches:
    our_enum_unmatched: list[EnumValue] = field(default_factory=list)
    their_enum_unmatched: list[str] = field(default_factory=list)
    matches: list[tuple[str, str]] = field(default_factory=list)


@dataclass
class ErrFromCode:
    code: str
    err_from_type: str


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

    def __err_gen(
        self, src_enum_name: str, dest_enum_name: str, enum_values: list[str]
    ) -> ErrFromCode:
        """
        Generate the error enum for the case where some variants of the
        source enum are not matched to any variant of the destination enum.
        """
        if len(enum_values) == 0:
            return ErrFromCode(code="", err_from_type="std::convert::Infallible")

        src_enum_as_ident = "".join(
            word.capitalize()
            for ident in src_enum_name.split("::")
            for word in ident.split("_")
        )
        err_from_type = f"ErrFrom{src_enum_as_ident}"

        return ErrFromCode(
            code=f"""
{self.__feature_gate_gen()}
/// All the variants of {src_enum_name} that are not matched to any variant of {dest_enum_name}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum {err_from_type} {{
    {"\n".join(f"            {enum_value}," for enum_value in enum_values)}
}}

{self.__feature_gate_gen()}
impl std::fmt::Display for {err_from_type} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match self {{
            {"\n".join(f'            {err_from_type}::{enum_value} => write!(f, "{enum_value} has no corresponding value in {dest_enum_name}"),' for enum_value in enum_values)}
        }}
    }}
}} 

{self.__feature_gate_gen()}
impl std::error::Error for {err_from_type} {{}}
""",
            err_from_type=err_from_type,
        )

    def __feature_gate_gen(self) -> str:
        return (
            f'#[cfg(feature = "{self.__feature_gate}")]' if self.__feature_gate else ""
        )

    def generate(
        self,
        enum_name: str,
        enum_values: list[EnumValue],
        version: VersionInfo,
        rs_file: str,
    ) -> Code[str]:
        matches = self.__matcher(enum_values)

        # our enum -> their enum. our enum is the input here.

        err_from = self.__err_gen(
            enum_name,
            self.__their_enum_rust_qualified_name,
            [ours.rust_identifier for ours in matches.our_enum_unmatched],
        )

        code_ours_to_theirs = f"""
{self.__feature_gate_gen()}
impl std::convert::TryFrom<{enum_name}> for {self.__their_enum_rust_qualified_name} {{
    type Error = {err_from.err_from_type};
    fn try_from(value: {enum_name}) -> Result<Self, Self::Error> {{
        match value {{
            {"\n".join(f"            {enum_name}::{ours} => Ok({self.__their_enum_rust_qualified_name}::{theirs})," for (ours, theirs) in matches.matches)}
            {"\n".join(f"            {enum_name}::{ours.rust_identifier} => Err({err_from.err_from_type}::{ours.rust_identifier})," for ours in matches.our_enum_unmatched)}
        }}
    }}
}}

{err_from.code}
"""

        err_from = self.__err_gen(
            self.__their_enum_rust_qualified_name,
            enum_name,
            matches.their_enum_unmatched,
        )

        code_theirs_to_ours = f"""
{self.__feature_gate_gen()}
impl std::convert::TryFrom<{self.__their_enum_rust_qualified_name}> for {enum_name} {{
    type Error = {err_from.err_from_type};
    fn try_from(value: {self.__their_enum_rust_qualified_name}) -> Result<{enum_name}, Self::Error> {{
        match value {{
            {"\n".join(f"            {self.__their_enum_rust_qualified_name}::{theirs} => Ok({enum_name}::{ours})," for (ours, theirs) in matches.matches)}
            {"\n".join(f"            {self.__their_enum_rust_qualified_name}::{theirs} => Err({err_from.err_from_type}::{theirs})," for theirs in matches.their_enum_unmatched)}
        }}
    }}
}}

{err_from.code}
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
