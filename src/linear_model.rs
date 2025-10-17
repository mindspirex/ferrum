pub struct LinearRegression {
  weights: Vec<Vec<f64>>,
  b: f64,
  alpha: f64,
  epochs: i64,
}

fn mat_mul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
  let rows = a.len();
  let cols = b[0].len();
  let inner = b.len();

  let mut result = vec![vec![0.0; cols]; rows];

  for i in 0..rows {
    for j in 0..cols {
      for k in 0..inner {
        result[i][j] += a[i][k] * b[k][j];
      }
    }
  }

  result
}

impl LinearRegression {
  pub fn new() -> Self {
    Self {
      weights: Vec::new(),
      b: 0.0,
      alpha: 0.1,
      epochs: 10,
    }
  }

  pub fn fit(&mut self, x: &[f64], y: &[f64]) {}

  pub fn predict(&self, x: &[f64]) -> Vec<f64> {
    let mut predictions: Vec<f64> = vec![0.0; x.len()];
    for i in 0..predictions.len() {
      predictions[i] = 0.0;
    }

    predictions
  }
}
