fn gauss_const(h: f64) -> f64 {
    let pi = std::f64::consts::PI * 2.;
    1. / (h * pi.sqrt())
}

fn kernel_function_vec(h: f64, ker_x: &Vec<f64>, xi: f64) -> Vec<f64> {
    ker_x.iter().map(|x| kernel_function(h, *x, xi)).collect()
}

fn gauss_exp(ker_x: f64, xi: f64, h: f64) -> f64 {
    -(0.5 * (xi - ker_x).powf(2.)) / h * h
}

fn kernel_function(h: f64, ker_x: f64, xi: f64) -> f64 {
    gauss_const(h) * gauss_exp(ker_x, xi, h).exp()
}

fn weights(bw_manual: f64, input_x: f64, all_input_values: &Vec<f64>) -> Vec<f64> {
    let mut w_row: Vec<f64> = vec![];
    for x_i in all_input_values {
        let ki = kernel_function(bw_manual, *x_i, input_x);
        let ki_sum: f64 = kernel_function_vec(bw_manual, all_input_values, input_x)
            .iter()
            .sum();

        w_row.push(ki / ki_sum);
    }
    w_row
}

pub fn kernel_regression(bw_manual: f64, input_x: f64, new_x: &Vec<f64>) -> f64 {
    let w = weights(bw_manual, input_x, &new_x);
    new_x.iter().zip(w.iter()).map(|(a, b)| (a * b)).sum()
}
