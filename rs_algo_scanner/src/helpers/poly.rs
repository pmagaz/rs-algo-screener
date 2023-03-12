use polyfit_rs::polyfit_rs::polyfit;

pub fn eval_polynomial(p: &[f64], x: &[f64]) -> Vec<f64> {
    let mut i = 0;
    let mut y: Vec<f64> = x.iter().map(|_x| 0.).collect();
    let mut result: Vec<f64> = vec![];
    for _x in p {
        let zu: Vec<f64> = x
            .iter()
            .zip(y.iter())
            .map(|(a, b)| (a * b) + p[i])
            .collect();
        y = zu.clone();
        i += 1;
        result = zu;
    }
    result
}

pub fn poly_fit(x_values: &[f64], y_values: &[f64], degree: usize) -> Vec<(usize, f64)> {
    let mut poly = polyfit(x_values, y_values, degree).unwrap();
    poly.reverse();
    let mut result: Vec<(usize, f64)> = vec![];
    let mut foo = 0;
    for i in x_values {
        let polyval = eval_polynomial(&poly, x_values);
        result.push((*i as usize, polyval[foo]));
        foo += 1;
    }
    result
}
