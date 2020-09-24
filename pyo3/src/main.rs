use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};


fn load_python(raw_code: &str){
    Python::with_gil(|py| {
        let activators = PyModule::from_code(py, raw_code)?;
    
        // ----- 
        let relu_result: f64 = activators.call1("relu", (-1.0,))?.extract()?;
        assert_eq!(relu_result, 0.0);
    
        let kwargs = [("slope", 0.2)].into_py_dict(py);
        let lrelu_result: f64 = activators
            .call("leaky_relu", (-1.0,), Some(kwargs))?
            .extract()?;
        assert_eq!(lrelu_result, -0.2);
    })
}


fn main() {
    println!("Hello, world!");
}
