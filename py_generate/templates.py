from dataclasses import dataclass
import re
import string
from typing import Literal, cast
import unicodedata

from py_generate.common import RS_TS, TS_DIR, VersionInfo

_words_re = re.compile(r"\b\w+\b", flags=re.UNICODE)

TAB = "    "


def normalize_nfkc(text: str) -> str:
    """Normalizes a string to NFKC form."""
    return unicodedata.normalize("NFKC", text)


def convert_to_identifier(name: str) -> str:
    # only use a-z, A-Z, 0-9, _
    # UpperCamelCase

    name = normalize_nfkc(name)

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

    def gen_enum_value_definition(self, ts_rs: Literal["ts", "rs"] = "rs") -> str:
        description = re.sub(r"\s+", " ", self.description)

        extras = ""
        for extra in self.extras or []:
            if not extra:
                continue

            extra = re.sub(r"\s+", " ", extra)
            if extra == description or extra == "nan":
                continue

            if ts_rs == "rs":
                extras += f"\n{TAB}///\n{TAB}/// {extra}"
            else:
                extras += f"\n{TAB}*\n{TAB}* {extra}"

        if ts_rs == "rs":
            return f"{TAB}/// {description}{extras}\n{TAB}{self.rust_identifier},"
        else:
            return f'{TAB}/**\n{TAB}* {description}{extras}\n{TAB}*/\n{TAB}{self.rust_identifier} = "{self.code}",'


def enum_generate(enum_name: str, enum_values: list[EnumValue]) -> RS_TS[str]:
    return RS_TS(
        rs=f"""
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum {enum_name} {{
{'\n'.join(enum_value.gen_enum_value_definition() for enum_value in enum_values)}
}}
""",
        ts=f"""
export enum {enum_name} {{
{'\n'.join(enum_value.gen_enum_value_definition('ts') for enum_value in enum_values)}
}}
""",
    )


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


def description_trait_generate(
    enum_name: str, enum_values: list[EnumValue]
) -> RS_TS[str]:
    pattern = re.compile(r"\s+", flags=re.UNICODE)

    return RS_TS(
        rs=f"""
impl crate::Description for {enum_name} {{
{TAB}fn description(&self) -> &str {{
{TAB}{TAB}match self {{
{'\n'.join(f"{TAB}{TAB}{TAB}{enum_name}::{enum_value.rust_identifier} => \"{pattern.sub(" ", enum_value.description)}\"," for enum_value in enum_values)}
{TAB}{TAB}}}
{TAB}}}
}}
""",
        ts=f"""
export function description(value: {enum_name}): string {{
{TAB}switch (value) {{
{'\n'.join(f"{TAB}{TAB}case {enum_name}.{enum_value.rust_identifier}: return \"{pattern.sub(" ", enum_value.description)}\";" for enum_value in enum_values)}
{TAB}}}
}}
""",
    )


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
    index_ts_file = TS_DIR / version.src_dir / "index.ts"
    index_ts_file.parent.mkdir(parents=True, exist_ok=True)
    with open(index_ts_file, "w") as f:
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
        f.write(code_definition.rs)
        f.write(code_trait_impl_code)
        f.write(code_trait_impl_description.rs)
        f.write(code_trait_impl_from_code)

    out_file_ts = TS_DIR / version.src_dir / rs_file
    out_file_ts = out_file_ts.with_suffix(".ts")
    out_file_ts.parent.mkdir(parents=True, exist_ok=True)
    with open(out_file_ts, "w") as f:
        f.write(code_definition.ts)
        f.write(code_trait_impl_description.ts)

    if write_mod:
        mod_file = version.src_dir / "mod.rs"
        with open(mod_file, "a") as f:
            f.write(f"pub mod {rs_file[:-3]};\n")
            f.write(f"pub use {rs_file[:-3]}::{enum_name};\n")
        index_ts_file = TS_DIR / version.src_dir / "index.ts"
        with open(index_ts_file, "a") as f:
            f.write(f"export {{ {enum_name} }} from './{rs_file[:-3]}';\n")
