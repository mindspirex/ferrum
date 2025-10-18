use ndarray::Array2;

pub struct LinearRegression {
  weights: Array2<f64>,
  learning_rate: f64,
  epochs: i64,
}

impl LinearRegression {
  pub fn new() -> Self {
    Self {
      weights: Array2::default((3, 3)),
      learning_rate: 0.1,
      epochs: 10,
    }
  }

  pub fn fit(&mut self, x: &Array2<f64>, y: &Array2<f64>) {
    for _epoch in 0..self.epochs {}
  }

  pub fn predict(&self, x: &Array2<f64>) -> Array2<f64> {
    let predictions = x.dot(&self.weights);
    predictions
  }
}
