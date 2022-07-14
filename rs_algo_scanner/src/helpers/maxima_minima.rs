use crate::error::Result;
use find_peaks::PeakFinder;
use std::cmp::Ordering;
use std::env;

pub fn maxima_minima(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    min_prominence: f64,
    min_distance: usize,
) -> Result<Vec<(usize, f64)>> {
    let logarithmic = env::var("LOGARITHMIC_SCANNER").unwrap().parse::<bool>().unwrap();
    let result: Vec<(usize, f64)> = PeakFinder::new(&x_values)
        .with_min_prominence(min_prominence)
        .with_min_distance(min_distance)
        .find_peaks()
        .iter()
        .map(|peak| {
            let x = peak.middle_position();
            let y = y_values[x];
            let y = match logarithmic {
                true => y.exp(),
                false => y
            };
            return (x, y);
        })
        .collect();
    Ok(result)
}

pub fn maxima_minima_exp(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    min_prominence: f64,
    min_distance: usize,
) -> Result<Vec<(usize, f64)>> {
    let result: Vec<(usize, f64)> = PeakFinder::new(&x_values)
        .with_min_prominence(min_prominence)
        .with_min_distance(min_distance)
        .find_peaks()
        .iter()
        .map(|peak| {
            let x = peak.middle_position();
            let y = y_values[x];
            return (x, y);
            //return (x, y);
        })
        .collect();
    Ok(result)
}

pub fn peaks_are_sorted<T: IntoIterator>(t: T) -> Ordering
where
    <T as IntoIterator>::Item: std::cmp::PartialOrd,
{
    let mut iter = t.into_iter();
    let mut len = 0;
    let mut ascendant = 0;
    let mut descendant = 0;
    if let Some(first) = iter.next() {
        iter.fold(first, |previous, current| {
            len += 1;
            if previous > current {
                descendant += 1;
            } else {
                ascendant += 1;
            }
            current
        });
    }

    if ascendant == len {
        Ordering::Greater
    } else if descendant == len {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
