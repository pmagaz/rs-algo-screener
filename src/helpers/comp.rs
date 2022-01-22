use std::env;

pub fn is_equal(x: f64, y: f64) -> bool {
    let threshold = env::var("EQUAL_THRESHOLD").unwrap().parse::<f64>().unwrap();
    let max = x.max(y);
    let min = y.min(x);
    let increase = max - min;
    let percentage_increase = (increase / x) * 100.;
    if percentage_increase > 0. && percentage_increase < threshold {
        true
    } else {
        false
    }
}

fn average(numbers: &[i32]) -> f64 {
    numbers.iter().sum::<i32>() as f64 / numbers.len() as f64
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
