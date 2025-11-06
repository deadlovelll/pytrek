import typer
import subprocess
from pathlib import Path

from . import pytrek

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
    if all:
        subprocess.run(['pytest', str(path)])

@app.command()
def init():
    result = pytrek.init_project()
    typer.echo(result)
    
@app.command()
def sync_project():
    result = pytrek.sync_project() 
    typer.echo(result)

@app.command()
def sync_hashes():
    result = pytrek.sync_file_hashes()
    typer.echo(result)

@app.command()
def sync_graph():
    result = pytrek.sync_dependency_graph()
    typer.echo(result)