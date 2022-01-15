pub use horner::eval_any_rank_polynomial;
use polyfit_rs::polyfit_rs::polyfit;
/*  for i in range(len(p)):
    y = y * x + p[i]
return y*/

// def polyval(p, x):
//   x = NX.asanyarray(x)
//   y = NX.zeros_like(x)
//   for i in range(len(p)):
//     y = y * x + p[i]
//   return y

// SEGUIR AQUI
pub fn leches(p: &[f64], x: &[f64]) -> Vec<f64> {
  let mut y: f64 = 0.;
  let mut i = 0;
  let mut y: Vec<f64> = x.iter().map(|x| 0.).collect();
  let mut result: Vec<f64> = vec![];
  for j in p {
    let zu: Vec<f64> = x.iter().zip(y.iter()).map(|(a, b)| (a * b) + p[i]).collect();
    //println!("33333 {:?} ,{:?} ", x, zu);
    y = zu.clone();
    i += 1;
    result = zu;
  }
  return result;
}

pub fn fit(x_values: &[f64], y_values: &[f64], degree: usize) -> Vec<(usize, f64)> {
  // let x_values = [4., 10., 64., 101., 154., 185., 220., 251.];
  // let y_values = [164.53, 161.0, 154.39, 147.09, 132.45, 124.715, 136.54, 134.3409];

  let mut poly = polyfit(&x_values, &y_values, degree).unwrap();
  poly.reverse();

  // [-1.830139091034475e-11, 1.1308171458819426e-8, -2.3893845027387917e-6, 0.0001934011463582165, -0.0038096164696380663, -0.2724572229690416, 164.92978475692624]
  // [-1.83328531e-11  1.13320863e-08 -2.39641203e-06  1.94397752e-04 -3.87832325e-03 -2.70506063e-01  1.64920538e+02]
  let mut result: Vec<(usize, f64)> = vec![];
  let mut foo = 0;
  for i in x_values {
    let polyval = eval_any_rank_polynomial::<f64>(*i as f64, &poly);
    result.push((*i as usize, polyval));
    // let polyval = leches(&poly, &x_values);
    // result.push((*i as usize, polyval[foo]));
    foo += 1;
  }
  // (4, 163.79077949707516), (10, 161.9948706973578), (64, 153.38445827009468), (101, 148.5935159054827), (154, 130.42654461755757), (185, 126.6636571477393), (220, 135.46808894424763), (237, 134.73293785150946)]
  // [163.78830033 161.9991938  153.38553106 148.59392421 130.42732562 126.66335219 135.46977197 134.72850081]

  result
}
