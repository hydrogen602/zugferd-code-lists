from typing import Annotated, Literal
import typer
import pandas as pd

from py_generate.sheets import run_all
from py_generate.common import load_all_sheets, Version, load_sheet
from py_generate.generate_exports import main as generate_exports_main
from py_generate.repeated_code import main as repeated_code_main

app = typer.Typer()


@app.command()
def all(
    version: Annotated[
        Version | None,
        typer.Argument(
            show_default=False,
            help="The version to generate for. If not provided, all versions will be generated.",
        ),
    ] = None,
):
    versions = sorted(list(Version))

    if version is None:
        prev_version_gen_info = None
        for v in versions:
            prev_version_gen_info = run_all(v.version_info(), prev_version_gen_info)
    else:
        run_all(version.version_info())


@app.command()
def list_sheets():
    for v in Version:
        version = v.version_info()
        typer.echo(f"Listing sheets for ZUGFeRD {version.version}")
        sheets = load_all_sheets(version).keys()
        for sheet in sheets:
            typer.echo(sheet)


@app.command()
def get_sheet(version: Version, sheet: str, header_idx: int = 0):
    v = version.version_info()
    sheets = load_all_sheets(v).keys()
    if sheet not in sheets:
        typer.echo(f"Sheet {sheet} not found")
        return
    sheet_df = load_sheet(sheet, v, header_idx)
    with pd.option_context("display.max_rows", None):
        typer.echo(sheet_df)


@app.command()
def generate_exports():
    generate_exports_main()


@app.command()
def repeated_code(lang: Literal["ts", "rs"]):
    repeated_code_main(lang)


if __name__ == "__main__":
    app()
