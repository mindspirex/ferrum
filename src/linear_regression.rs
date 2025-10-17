pub struct LinearRegression {
  weights: Vec<i64>,
}

impl LinearRegression {
  pub fn new() -> Self {
    Self {
      weights: Vec::new(),
    }
  }

  pub fn fit(&mut self, x: &[f64], y: &[f64]) {
    // placeholder logic
    self.weights = vec![0.0; x.len()];
  }

  pub fn predict(&self, x: &[f64]) -> f64 {
    self.weights.iter().zip(x).map(|(w, xi)| w * xi).sum()
  }
}
