use pyo3::prelude::*;

pub mod file_hasher;
pub mod graph_creator;
pub mod project_initializer;

use crate::file_hasher::FileHasher;
use crate::graph_creator::GraphCreator;
use crate::project_initializer::ProjectInitializer;


#[pyfunction]
fn init_project() -> PyResult<String> {
    let file_hasher = FileHasher::new();
    let graph_creator = GraphCreator::new();
    let project_initializer = ProjectInitializer::new(file_hasher, graph_creator);
    project_initializer.init();
    Ok("Project initialized".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pytrek(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_project)?)?;
    Ok(())
}
