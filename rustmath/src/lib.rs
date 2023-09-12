use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict};

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyclass]
struct MyClass {}

#[pymethods]
impl MyClass {
    #[staticmethod]
    #[args(kwargs = "**")]
    fn test1(kwargs: Option<&PyDict>) -> PyResult<()> {
        if let Some(kwargs) = kwargs {
            for kwarg in kwargs {
                println!("{:?}", kwarg);
            }
        } else {
            println!("kwargs is none");
        }
        Ok(())
    }

    #[staticmethod]
    #[args(args = "*")]
    fn test2(args: &PyTuple) -> PyResult<()> {
        for arg in args {
            println!("{:?}", arg);
        }
        Ok(())
    }
}
#[pyclass]
pub struct RustStruct {
    #[pyo3(get, set)]
    pub data: String,
    #[pyo3(get, set)]
    pub vector: Vec<u8>,
}
#[pymethods]
impl RustStruct {
    #[new]
    pub fn new(data: String, vector: Vec<u8>) -> RustStruct {
        RustStruct { data, vector }
    }
    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }
    pub fn extend_vector(&mut self, extension: Vec<u8>) {
        println!("{}", self.data);
        for i in extension {
            self.vector.push(i);
        }
    }
}
#[pymodule]
fn rustmath(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_class::<RustStruct>()?;
    Ok(())
}