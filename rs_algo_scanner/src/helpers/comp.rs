use std::env;

//FIXME move to shared
pub fn percentage_change(x: f64, y: f64) -> f64 {
    let max = x.max(y);
    let min = y.min(x);
    let increase = max - min;
    let percentage_move = (increase / x) * 100.;
    percentage_move
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

pub fn is_equal_distance(a: (f64, f64), b: (f64, f64)) -> bool {
    let move_a = (a.0 - a.1).abs();
    let percentage_move_a = (move_a / b.1) * 100.;

    let move_b = (b.0 - b.1).abs();
    let percentage_move_b = (move_b / b.1) * 100.;

    let threshold = env::var("EQUAL_DISTANCE_THRESHOLD")
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if (percentage_move_a - percentage_move_b).abs() < threshold {
        true
    } else {
        false
    }
}

pub fn average(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() as f64 / numbers.len() as f64
}
