import typer
from pathlib import Path

app = typer.Typer(help='pytrek')

@app.command()
def run(
    all: bool = typer.Option(
        False,
        help='Run all tests in project'
    ),
    no_sync: bool = typer.Option(
        False, 
        help='Run tests without graph synchronization',
    ),
    path: Path = typer.Option(
        Path('.'),
        '--path',
        '-p',
        help='Root path of the project'
    ),
):
    pass