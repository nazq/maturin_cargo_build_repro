use pyo3::exceptions::PyFileNotFoundError;

use pyo3::prelude::*;

use std::path::{Path, PathBuf};

fn simple_call() -> String {
    "simple result".to_string()
}

fn find_file(file_path: &Path) -> PyResult<PathBuf> {
    let current_path = file_path.to_path_buf();

    if current_path.is_file() {
        return Ok(current_path);
    } else {
        return Err(PyFileNotFoundError::new_err(format!(
            "File not found: {}",
            current_path.display()
        )));
    }
}

#[pyfunction]
pub fn py_check_file(file_path: &str) -> PyResult<String> {
    let path = Path::new(file_path);
    let result = find_file(path)?;
    Ok(result.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_simple_call() {
        let expected = "simple result".to_string();
        let result = simple_call();
        assert_eq!(result, expected);
    }
    
        #[test]
        fn test_find_cfg_file_in_current_directory() {
            let result = find_file(Path::new("Cargo.toml")).unwrap();
            assert_eq!(result, PathBuf::from("Cargo.toml"));
        }

        #[test]
        fn test_py_check_file() {
            let result = py_check_file("Cargo.toml").unwrap();
            assert_eq!(result, "Cargo.toml");
        }

        #[test]
        fn test_py_check_file_fail() {
            let result = py_check_file("Cargo.toml.NO");
            assert!(result.is_err());
        }
}
