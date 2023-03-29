use pyo3::prelude::*;
use std::time::{Duration, SystemTime};
use rust_search::{FileSize, FilterExt, SearchBuilder};

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring

/// A package for searching files
///
/// Basic usage:
///
/// >>> get_similar_files('png', './')
/// []
///
#[pymodule]
fn py_rust_search(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn get_similar_files(search_input: String, search_dir: String) -> PyResult<Vec<String>> {
        let mut search: Vec<String> = ::rust_search::SearchBuilder::default()
            .location(&search_dir)
            .search_input(&search_input)
            .depth(1)
            .ignore_case()
            .build()
            .collect();
        ::rust_search::similarity_sort(&mut search, &search_input);
        Ok(search)
    }

    #[pyfn(m)]
    fn get_all_ext(search_ext: String, search_dir: String) -> PyResult<Vec<String>> {
        let files: Vec<String> = ::rust_search::SearchBuilder::default()
        .location(&search_dir)
        .ext(&search_ext)
        .build()
        .collect();
        Ok(files)
    }

    #[pyfn(m)]
    fn get_all_filter(search_dir: String) -> PyResult<Vec<String>> {
        let search: Vec<String> = SearchBuilder::default()
		.location(&search_dir)
		.file_size_greater(FileSize::Megabyte(1.0))
		.file_size_smaller(FileSize::Megabyte(10.0))
		.created_after(SystemTime::now() - Duration::from_secs(3600 * 24 * 10))
		.created_before(SystemTime::now())
		.modified_after(SystemTime::now() - Duration::from_secs(3600 * 24 * 5))
		// .custom_filter(|dir| dir.metadata().unwrap().is_file())
		// .custom_filter(|dir| !dir.metadata().unwrap().permissions().readonly())
		.build()
		.collect();
        Ok(search)
    }

    Ok(())
}
