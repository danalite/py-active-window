use pyo3::prelude::*;
use pyo3::types::PyDict;
use active_win_pos_rs::{get_active_window};

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring

// ActiveWindow {
//     title: "cmd - cargo  run --example active-window",
//     process_path: "C:\\Program Files\\WindowsApps\\...",
//     app_name: "WindowsTerminal",
//     window_id: "HWND(9700584)",
//     process_id: 8460,
//     position: WindowPosition {
//         x: 6.0,
//         y: 296.0,
//         width: 1129.0,
//         height: 635.0,
//     },
// }

/// A package for searching files
///
/// Basic usage:
///
/// get_active_win()
/// []
///
#[pymodule]
fn py_active_win(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn get_active_win(py: Python) -> PyResult<Py<PyDict>> {
        let py_win = PyDict::new(py);
        match get_active_window() {
            Ok(active_window) => {
                // println!("active window: {:#?}", active_window);
                // Ok(active_window)
                py_win.set_item("app_name", active_window.app_name)?;
                py_win.set_item("title", active_window.title)?;
                py_win.set_item("process_id", active_window.process_id)?;
                py_win.set_item("process_path", active_window.process_path)?;
                py_win.set_item("window_id", active_window.window_id)?;
                py_win.set_item("x", active_window.position.x)?;
                py_win.set_item("y", active_window.position.y)?;
                py_win.set_item("width", active_window.position.width)?;
                py_win.set_item("height", active_window.position.height)?;
                py_win
            },
            Err(()) => {
                println!("error occurred while getting the active window");
                py_win
            }
        };
        Ok(py_win.into())
    }
    Ok(())
}
