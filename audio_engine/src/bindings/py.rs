use pyo3::prelude::*;

#[pyfunction]
pub fn render(num_samples: usize, channels: usize, sample_rate: usize) -> Vec<f32> {
    crate::render(num_samples, channels, sample_rate)
}

#[pymodule]
fn audio_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render, m)?)?;

    Ok(())
}