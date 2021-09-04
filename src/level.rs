use bevy::math::Vec2;

/// A Level contains all the information needed to setup the plot
pub struct Level {
    /// Coefficients of the polynomial that describes the path
    pub coefs: Vec<f64>,
    /// Used to determine which area of the plot to show
    pub limits: [Vec2; 2],
    /// Time in seconds to complete the game.
    pub time: f64,
    start_x: f64,
    end_x: f64,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            coefs: vec![-1.0, 0.0, 1.0],
            limits: [Vec2::new(-2., -2.), Vec2::new(2., 2.)],
            time: 100.0,
            start_x: -1.0,
            end_x: 1.0,
        }
    }
}
impl Level {
    pub fn eval_poly(&self, x: f64) -> f64 {
        let mut y = 0.;
        for coef in &self.coefs {
            y = coef + x * y;
        }
        y
    }
}
