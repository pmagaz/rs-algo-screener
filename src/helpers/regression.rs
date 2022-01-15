use friedrich::kernel;

fn gauss_const(h: f64) -> f64 {
    let pi = std::f64::consts::PI * 2.;
    1. / (h * pi.sqrt())
}

fn gauss_exp(ker_x: &Vec<f64>, xi: f64, h: f64) -> Vec<f64> {
    let num: Vec<f64> = ker_x
        .iter()
        .map(|x| (0.5 * (xi - x).powf(2.)) / h * h)
        .collect();
    num
}

fn kernel_function(h: f64, ker_x: &Vec<f64>, xi: f64) -> Vec<f64> {
    let gauss_const = gauss_const(h);
    let gauss_val: Vec<f64> = gauss_exp(ker_x, xi, h)
        .iter()
        .map(|x| gauss_const * x.exp())
        .collect();
    println!("{:?}", gauss_val);
    //println!("11111, {:?}", gauss_val.par_map_inplace(|x| *x = x.exp()));
    gauss_val
}

// fn weights(bw_manual: f64, input_x: f64, all_input_values: Vec<f64>) {
//     for x in all_input_values {
//         let ki = kernel_function(bw_manual, all_input_values, input_x);
//     }
// }

pub fn kernel_regression(kernel_x: &Vec<f64>, input_x: f64) -> Vec<f64> {
    let bw_manual = 1.0;
    let col3 = kernel_function(bw_manual, kernel_x, input_x);
    col3
}
