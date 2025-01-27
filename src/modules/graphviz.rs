use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::ffi::CString;

pub fn plotting_graphviz(dot_code: &str, output_file_stem: &str) -> PyResult<()> {
    // Инициализируем интерпретатор Python и выполняем внутри GIL-блок
    Python::with_gil(|py| {
        // Импортируем модуль `graphviz`
        let graphviz = PyModule::import(py, "graphviz")?;
        let render_code = format!(
            "from graphviz import Source\n\
             s = Source(r\"\"\"{dot_code}\"\"\")\n\
             s.render(filename=r\"{file_stem}\", format='png', cleanup=True)\n",
            dot_code = dot_code,
            file_stem = output_file_stem,
        );
            py.run(&CString::new(render_code).expect("Can't run python code"), None, Some(&graphviz.dict()))?;

        Ok(())
    })
}