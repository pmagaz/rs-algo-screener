fn gauss_const(h: f64) -> f64 {
    let pi = std::f64::consts::PI * 2.;
    1. / (h * pi.sqrt())
}

fn gauss_exp(ker_x: &Vec<f64>, xi: f64, h: f64) -> Vec<f64> {
    let den = h * h;
    let num: Vec<f64> = ker_x
        .iter()
        .map(|x| (0.5 * (xi - x).sqrt()) / den)
        .collect();
    num
}

fn kernel_function(h: f64, ker_x: &Vec<f64>, xi: f64) -> Vec<f64> {
    let gauss_const = gauss_const(h);
    let gauss_val: Vec<f64> = gauss_exp(ker_x, xi, h)
        .iter()
        .map(|x| x.exp() * gauss_const)
        .collect();
    gauss_val
}

pub fn kernel_regression(kernel_x: &Vec<f64>) {
    let bw_manual = 1.0;
    let input_x = kernel_x[0];
    let col1 = gauss_const(bw_manual);
    let col2 = gauss_exp(kernel_x, input_x, bw_manual);
    let col3 = kernel_function(bw_manual, kernel_x, input_x);
}
