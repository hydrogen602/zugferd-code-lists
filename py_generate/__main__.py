import typer
from py_generate.sheets import run_all
from py_generate.common import load_all_sheets, ZF_232

app = typer.Typer()


@app.command()
def all():
    run_all()


@app.command()
def list_sheets():
    sheets = load_all_sheets(ZF_232).keys()
    for sheet in sheets:
        typer.echo(sheet)


if __name__ == "__main__":
    app()
