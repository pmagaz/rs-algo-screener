use std::env;

fn gauss_const(h: f64) -> f64 {
    let pi = std::f64::consts::PI * 2.;
    1. / (h * pi.sqrt())
}

fn gauss_exp(x: f64, y: f64, h: f64) -> f64 {
    let den = h * h;
    let num = -0.5 * (y - x).powf(2.);
    num / den
}

fn kernel_function_vec(h: f64, y: f64, x: &Vec<f64>, logarithmic: bool) -> Vec<f64> {
    x.iter().map(|x| kernel_function(h, y, *x, logarithmic)).collect()
}

fn kernel_function(h: f64, x: f64, y: f64, logarithmic: bool) -> f64 {

    let gauss_exp = match logarithmic {
        true => gauss_exp(x, y, h).exp(),
        false => gauss_exp(x, y, h)
    };
    gauss_const(h) * gauss_exp
}

fn weights(bandwidth: f64, x: f64, data: &Vec<f64>, logarithmic: bool) -> Vec<f64> {
    let mut w_row: Vec<f64> = vec![];
    let kernel_sum: f64 = kernel_function_vec(bandwidth, x, data, logarithmic).iter().sum();
    for x_i in data {
        let ki = kernel_function(bandwidth, x, *x_i, logarithmic);
        w_row.push(ki / kernel_sum);
    }
    w_row
}

pub fn kernel_regression(bandwidth: f64, x: f64, data: &Vec<f64>) -> f64 {
    let logarithmic = env::var("LOGARITHMIC_SCANNER").unwrap().parse::<bool>().unwrap();
    let w = weights(bandwidth, x, data, logarithmic);
    data.iter().zip(w.iter()).map(|(a, b)| (a * b)).sum()
}
