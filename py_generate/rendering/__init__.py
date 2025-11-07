from __future__ import annotations

from dataclasses import dataclass
from abc import ABC, abstractmethod
import re
from typing import Literal

from py_generate.common import Code, VersionInfo


class CodeGenerator(ABC):
    @abstractmethod
    def generate(
        self,
        enum_name: str,
        enum_values: list[EnumValue],
        version: VersionInfo,
        rs_file: str,
    ) -> Code[str]: ...


TAB = "    "


@dataclass
class EnumValue:
    rust_identifier: str
    description: str
    code: str
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
