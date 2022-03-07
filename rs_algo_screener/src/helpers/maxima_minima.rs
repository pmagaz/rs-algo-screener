use crate::error::Result;
use find_peaks::PeakFinder;
use std::env;

pub fn maxima_minima(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    min_prominence: f64,
    max_value: &f64,
) -> Result<Vec<(usize, f64)>> {
    let prominence = max_value * min_prominence;
    let min_distance = env::var("MARKERS_DISTANCE")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let result: Vec<(usize, f64)> = PeakFinder::new(&x_values)
        .with_min_prominence(prominence)
        .with_min_distance(min_distance)
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
