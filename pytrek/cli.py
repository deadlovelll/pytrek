import typer
import subprocess
from pathlib import Path

from pytrek import (
    init_project,
    sync_project,
    sync_file_hashes,
    sync_dependency_graph,
)

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
    init_project()
    
@app.command()
def sync_project():
    sync_project()

@app.command()
def sync_hashes():
    sync_file_hashes()

@app.command()
def sync_graph():
    sync_dependency_graph()