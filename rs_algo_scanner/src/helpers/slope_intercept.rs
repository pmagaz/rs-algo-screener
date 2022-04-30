use rs_algo_shared::models::pattern::DataPoints;

pub fn slope_intercept(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    let y = y2 - y1;
    let x = x2 - x1;
    let slope = y / x;
    let y_intercept = y1 - (slope * x1);
    return (slope, y_intercept);
}

pub fn next_intercept(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    let (slope, y_intercept) = slope_intercept(x1, y1, x2, y2);
    let hypo = hypotenuse(x2 - x1, y2 - y1);
    let new_x1 = x2 + hypo;

    let new_y1 = (slope * new_x1) + y_intercept;

    (new_x1, new_y1)
}

fn hypotenuse(x: f64, y: f64) -> f64 {
    let num = x.powi(2) + y.powi(2);
    num.powf(0.5)
}

pub fn next_top_point(data: &DataPoints) -> (usize, f64) {
    let (next_index, next_price) =
        next_intercept(data[0].0 as f64, data[0].1, data[2].0 as f64, data[2].1);
    (next_index as usize, next_price)
}

pub fn next_bottom_point(data: &DataPoints) -> (usize, f64) {
    let (next_index, next_price) =
        next_intercept(data[1].0 as f64, data[1].1, data[3].0 as f64, data[3].1);
    (next_index as usize, next_price)
}

pub fn add_next_top_points(mut data: DataPoints) -> Vec<(usize, f64)> {
    data.push(next_bottom_point(&data));
    data.push(next_top_point(&data));
    data
}

pub fn add_next_bottom_points(mut data: DataPoints) -> Vec<(usize, f64)> {
    data.push(next_top_point(&data));
    data.push(next_bottom_point(&data));
    data
}
