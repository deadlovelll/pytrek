use pyo3::prelude::*;

pub mod core;

use crate::core::project_intializer::file_hasher::FileHasher;
use crate::core::project_intializer::graph_creator::GraphCreator;
use crate::core::project_intializer::config_creator::ConfigCreator;
use crate::core::project_intializer::project_initializer::ProjectInitializer;


#[pyfunction]
fn init_project() -> PyResult<String> {
    let file_hasher = FileHasher::new();
    let graph_creator = GraphCreator::new();
    let config_creator = ConfigCreator::new();
    let mut project_initializer = ProjectInitializer::new(
        file_hasher, 
        graph_creator, 
        config_creator,
    );
    project_initializer.init();
    Ok("Project initialized".to_string())
}

#[pyfunction]
fn sync_project() -> PyResult<String> {
    Ok("Project synchronized".to_string())
}

#[pyfunction]
fn sync_file_hashes() -> PyResult<String> {
    Ok("File hashes synchronized".to_string())
}

#[pyfunction]
fn sync_dependency_graph() -> PyResult<String> {
    Ok("Graph synchronized".to_string())
}

#[pymodule]
fn pytrek(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_project, m)?)?;
    m.add_function(wrap_pyfunction!(sync_project, m)?)?;
    m.add_function(wrap_pyfunction!(sync_file_hashes, m)?)?;
    m.add_function(wrap_pyfunction!(sync_dependency_graph, m)?)?;
    Ok(())
}
