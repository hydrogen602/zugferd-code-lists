from dataclasses import dataclass
import re
import string
from typing import cast

from py_generate.common import VersionInfo

_words_re = re.compile(r"\b\w+\b", flags=re.UNICODE)

TAB = "    "


def convert_to_identifier(name: str) -> str:
    # only use a-z, A-Z, 0-9, _
    # UpperCamelCase

    name = name.strip().replace("'", "")

    # step 1: extract all words with regex
    # step 2: capitalize the first letter of each word
    # step 3: join all words together

    words = cast(list[str], _words_re.findall(name))
    words = [word.capitalize() for word in words if word not in {"of", "the"}]
    identifier = "".join(words)
    # add a _ if it starts with a number
    if identifier[0] in string.digits:
        identifier = f"_{identifier}"
    return identifier


@dataclass
class EnumValue:
    rust_identifier: str
    description: str
    code: string
    extras: list[str] | None = None

    def gen_enum_value_definition(self) -> str:
        extras = ""
        for extra in self.extras or []:
            if not extra:
                continue

            extra = re.sub(r"\s+", " ", extra)
            extras += f"\n\n{TAB}/// {extra}"

        description = re.sub(r"\s+", " ", self.description)
        return f"{TAB}/// {description}{extras}\n{TAB}{self.rust_identifier},"


def enum_generate(enum_name: str, enum_values: list[EnumValue]) -> str:
    return f"""
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum {enum_name} {{
{'\n'.join(enum_value.gen_enum_value_definition() for enum_value in enum_values)}
}}
"""


def code_trait_generate(enum_name: str, enum_values: list[EnumValue]) -> str:
    return f"""
impl crate::Code for {enum_name} {{
{TAB}fn code(&self) -> &str {{
{TAB}{TAB}match self {{
{'\n'.join(f"{TAB}{TAB}{TAB}{enum_name}::{enum_value.rust_identifier} => \"{enum_value.code}\"," for enum_value in enum_values)}
{TAB}{TAB}}}
{TAB}}}
}}
"""


def description_trait_generate(enum_name: str, enum_values: list[EnumValue]) -> str:
    return f"""
impl crate::Description for {enum_name} {{
{TAB}fn description(&self) -> &str {{
{TAB}{TAB}match self {{
{'\n'.join(f"{TAB}{TAB}{TAB}{enum_name}::{enum_value.rust_identifier} => \"{enum_value.description}\"," for enum_value in enum_values)}
{TAB}{TAB}}}
{TAB}}}
}}
"""


"""
pub trait FromCode {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized;
}
"""


def from_code_trait_generate(enum_name: str, enum_values: list[EnumValue]) -> str:
    return f"""
impl crate::FromCode for {enum_name} {{
{TAB}fn from_code(code: &str) -> Option<Self>
{TAB}where
{TAB}{TAB}Self: Sized
{TAB}{{
{TAB}{TAB}match code {{
{'\n'.join(f"{TAB}{TAB}{TAB}\"{enum_value.code}\" => Some({enum_name}::{enum_value.rust_identifier})," for enum_value in enum_values)}
{TAB}{TAB}{TAB}_ => None,
{TAB}{TAB}}}
{TAB}}}
}}
"""


def reset_mod_rs(version: VersionInfo):
    version.src_dir.mkdir(parents=True, exist_ok=True)
    mod_file = version.src_dir / "mod.rs"
    with open(mod_file, "w") as f:
        f.write("")


def generate(
    enum_name: str,
    enum_values: list[EnumValue],
    version: VersionInfo,
    rs_file: str,
    write_mod: bool = False,
):
    code_definition = enum_generate(enum_name, enum_values)

    code_trait_impl_code = code_trait_generate(enum_name, enum_values)

    code_trait_impl_description = description_trait_generate(enum_name, enum_values)

    code_trait_impl_from_code = from_code_trait_generate(enum_name, enum_values)

    version.src_dir.mkdir(parents=True, exist_ok=True)

    out_file = version.src_dir / rs_file
    with open(out_file, "w") as f:
        f.write("#![allow(non_camel_case_types)]\n")
        f.write(code_definition)
        f.write(code_trait_impl_code)
        f.write(code_trait_impl_description)
        f.write(code_trait_impl_from_code)

    if write_mod:
        mod_file = version.src_dir / "mod.rs"
        with open(mod_file, "a") as f:
            f.write(f"pub mod {rs_file[:-3]};\n")
            f.write(f"pub use {rs_file[:-3]}::{enum_name};\n")
