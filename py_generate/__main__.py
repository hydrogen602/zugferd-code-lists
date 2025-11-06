from typing import Annotated
import typer
from py_generate.sheets import run_all
from py_generate.common import load_all_sheets, Version, load_sheet

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
    if version is None:
        for v in Version:
            run_all(v.version_info())
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
    typer.echo(sheet_df)


if __name__ == "__main__":
    app()
