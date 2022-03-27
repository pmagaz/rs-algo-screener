use std::env;
//FIXME move to shared
pub fn percentage_change(x: f64, y: f64) -> f64 {
    let max = x.max(y);
    let min = y.min(x);
    let increase = max - min;
    let percentage_increase = (increase / x) * 100.;
    percentage_increase
}

pub fn is_equal(x: f64, y: f64) -> bool {
    let threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let percentage_change = percentage_change(x, y);
    if percentage_change > 0. && percentage_change < threshold {
        true
    } else {
        false
    }
}

pub fn increase_equally(a: (f64, f64), b: (f64, f64)) -> bool {
    let increase_a = (a.0 - a.1).abs();
    let percentage_increase_a = (increase_a / b.1) * 100.;

    let increase_b = (b.0 - b.1).abs();
    let percentage_increase_b = (increase_b / b.1) * 100.;

    let threshold = env::var("INCREASE_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if (percentage_increase_a - percentage_increase_b).abs() < threshold {
        true
    } else {
        false
    }
}

pub fn average(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() as f64 / numbers.len() as f64
}

// pub fn median(numbers: &mut [f64]) -> f64 {
//     numbers.sort();
//     let mid = numbers.len() / 2;
//     numbers[mid]
// }
