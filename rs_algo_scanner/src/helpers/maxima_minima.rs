use crate::error::Result;
use find_peaks::PeakFinder;
use std::env;

pub fn maxima_minima(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    min_prominence: f64,
    min_distance: usize,
) -> Result<Vec<(usize, f64)>> {
    let result: Vec<(usize, f64)> = PeakFinder::new(&x_values)
        .with_min_prominence(min_prominence)
        //.with_min_distance(0)
        .find_peaks()
        .iter()
        .map(|peak| {
            let x = peak.middle_position();
            let y = y_values[x];
            return (x, y);
        })
        .collect();
    Ok(result)
}
