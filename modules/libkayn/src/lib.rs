use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod operations;
mod transformations;

type Pixel = [u8; 3];
type ColorInt = u32;

#[pyfunction]
fn grayscale(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::grayscale(image))
}

#[pyfunction]
fn negative(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::negative(image))
}

#[pyfunction]
fn convolute(
    image: Vec<Pixel>,
    mask: Vec<f32>,
    width: u32,
    height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(operations::convolute(image, mask, width, height))
}
#[pyfunction]
fn sobel(image: Vec<Pixel>, width: u32, height: u32) -> PyResult<Vec<ColorInt>> {
    Ok(operations::sobel(image, width, height))
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

#[pyfunction]
fn limiarize(image: Vec<Pixel>, threshold: u8) -> PyResult<Vec<ColorInt>> {
    Ok(operations::limiarize(image, threshold))
}

#[pyfunction]
fn binarize(image: Vec<Pixel>, threshold: u8) -> PyResult<Vec<ColorInt>> {
    Ok(operations::binarize(image, threshold))
}
#[pyfunction]
fn equalize(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::equalize(image))
}
#[pyfunction]
fn gray_to_color_scale(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::gray_to_color_scale(image))
}
#[pyfunction]
fn noise_reduction_max(
    image: Vec<Pixel>,
    distance: u32,
    width: u32,
    height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(operations::noise_reduction_max(
        image, distance, width, height,
    ))
}
#[pyfunction]
fn noise_reduction_min(
    image: Vec<Pixel>,
    distance: u32,
    width: u32,
    height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(operations::noise_reduction_min(
        image, distance, width, height,
    ))
}
#[pyfunction]
fn noise_reduction_midpoint(
    image: Vec<Pixel>,
    distance: u32,
    width: u32,
    height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(operations::noise_reduction_midpoint(
        image, distance, width, height,
    ))
}
#[pyfunction]
fn otsu_threshold(image: Vec<Pixel>, width: u32, height: u32) -> PyResult<u8> {
    Ok(operations::otsu_thresholding(image, width, height))
}

#[pyfunction]
fn dct(image: Vec<Pixel>, width: u32, height: u32) -> PyResult<(Vec<ColorInt>, Vec<f32>)> {
    Ok(transformations::dct_multithread(image, width, height))
}

#[pyfunction]
fn idct(coefficients: Vec<f32>, width: u32, height: u32) -> PyResult<Vec<ColorInt>> {
    Ok(transformations::idct_multithread(
        coefficients,
        width,
        height,
    ))
}

#[pyfunction]
fn resize_nn(
    image: Vec<Pixel>,
    width: u32,
    height: u32,
    new_width: u32,
    new_height: u32,
) -> PyResult<Vec<ColorInt>> {
    Ok(transformations::resize_nearest_neighbor(
        image, width, height, new_width, new_height,
    ))
}
#[pyfunction]
fn freq_lowpass(
    image: Vec<f32>,
    width: u32,
    height: u32,
    radius: u32,
) -> PyResult<(Vec<ColorInt>, Vec<f32>)> {
    Ok(transformations::freq_lowpass(image, width, height, radius))
}
#[pyfunction]
fn freq_highpass(
    image: Vec<f32>,
    width: u32,
    height: u32,
    radius: u32,
) -> PyResult<(Vec<ColorInt>, Vec<f32>)> {
    Ok(transformations::freq_highpass(image, width, height, radius))
}
#[pyfunction]
fn freq_normalize(image: Vec<f32>) -> PyResult<Vec<ColorInt>> {
    Ok(transformations::freq_normalize(image))
}

#[pyfunction]
fn equalize_hsl(image: Vec<Pixel>) -> PyResult<Vec<ColorInt>> {
    Ok(operations::equalize_hsl(image))
}
#[pyfunction]
fn split_color_channel(image: Vec<Pixel>, channel: usize) -> PyResult<Vec<ColorInt>> {
    Ok(operations::split_color_channel(image, channel))
}

#[pymodule]
fn libkayn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grayscale, m)?)?;
    m.add_function(wrap_pyfunction!(negative, m)?)?;
    m.add_function(wrap_pyfunction!(convolute, m)?)?;
    m.add_function(wrap_pyfunction!(sobel, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_compression, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    m.add_function(wrap_pyfunction!(limiarize, m)?)?;
    m.add_function(wrap_pyfunction!(binarize, m)?)?;
    m.add_function(wrap_pyfunction!(equalize, m)?)?;
    m.add_function(wrap_pyfunction!(gray_to_color_scale, m)?)?;
    m.add_function(wrap_pyfunction!(noise_reduction_max, m)?)?;
    m.add_function(wrap_pyfunction!(noise_reduction_min, m)?)?;
    m.add_function(wrap_pyfunction!(noise_reduction_midpoint, m)?)?;
    m.add_function(wrap_pyfunction!(otsu_threshold, m)?)?;
    m.add_function(wrap_pyfunction!(dct, m)?)?;
    m.add_function(wrap_pyfunction!(idct, m)?)?;
    m.add_function(wrap_pyfunction!(resize_nn, m)?)?;
    m.add_function(wrap_pyfunction!(freq_lowpass, m)?)?;
    m.add_function(wrap_pyfunction!(freq_highpass, m)?)?;
    m.add_function(wrap_pyfunction!(freq_normalize, m)?)?;
    m.add_function(wrap_pyfunction!(equalize_hsl, m)?)?;
    m.add_function(wrap_pyfunction!(split_color_channel, m)?)?;
    Ok(())
}
