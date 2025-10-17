pub struct LinearRegression {
  m: f64,
  b: f64,
  alpha: f64,
  epochs: i64,
}

impl LinearRegression {
  pub fn new() -> Self {
    Self {
      m: 0.0,
      b: 0.0,
      alpha: 0.1,
      epochs: 10,
    }
  }

  pub fn fit(&mut self, x: &[f64], y: &[f64]) {}

  pub fn predict(&self, x: &[f64]) -> f64 {
    let mut predictions: Vec<f64> = Vec::with_capacity(x.len());
    for (xi, i) in x.iter().enumerate() {
      predictions[i] = m * xi + b;
    }
    return predictions;
  }
}
