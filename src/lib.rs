extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(test_bind_lib, |py, m| {
    m.add(py, "__doc__", "What a cool rust code")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    Ok(())
});

fn get_result(_py: Python, val: &str) -> PyResult<String> {
    Ok("From rust: ".to_owned() + val)
}
