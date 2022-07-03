use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod operations;
type Pixel = [u8; 3];
type ColorInt = u32;
#[pyfunction]
fn grayscale(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::grayscale(image as Vec<Pixel>))
}

#[pyfunction]
fn negative(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::negative(image as Vec<Pixel>))
}

#[pyfunction]
fn filter_nxn(
    image: Vec<Pixel>,
    filter: Vec<f32>,
    width: u32,
    height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(operations::filter_nxn(image, filter, width, height))
}

#[pyfunction]
fn median(image: Vec<Pixel>, distance: u32, width: u32, height: u32) -> PyResult<Vec<ColorInt>> {
    Ok(operations::median(image, distance, width, height))
}

#[pyfunction]
fn dynamic_compression(image: Vec<Pixel>, constant: f32, gamma: f32) -> PyResult<Vec<ColorInt>> {
    Ok(operations::dynamic_compression(image, constant, gamma))
}

#[pyfunction]
fn normalize(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::normalize(image))
}

#[pymodule]
fn libkayn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grayscale, m)?)?;
    m.add_function(wrap_pyfunction!(negative, m)?)?;
    m.add_function(wrap_pyfunction!(filter_nxn, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_compression, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    Ok(())
}
