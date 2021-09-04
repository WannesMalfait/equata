use bevy::math::Vec2;

/// A Level contains all the information needed to setup the plot
pub struct Level {
    /// Coefficients of the polynomial that describes the path of the enemy
    pub enemy_coefs: Vec<f64>,
    /// Coefficients of the polynomial that describes the path of the player
    pub player_coefs: Vec<f64>,
    /// Used to determine which area of the plot to show
    pub limits: [Vec2; 2],
    /// Time in seconds to complete the game.
    pub max_time: f64,
    /// Time in seconds that have passed since the start of the level.
    pub time_taken: f64,
    start_x: f64,
    end_x: f64,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            enemy_coefs: vec![-1.0, 0.0, 1.0],
            player_coefs: vec![1.0; 3],
            limits: [Vec2::new(-2., -1.), Vec2::new(2., 2.)],
            max_time: 100.0,
            time_taken: 0.0,
            start_x: -1.0,
            end_x: 1.0,
        }
    }
}
impl Level {
    pub fn eval_enemy_poly(&self, x: f64) -> f64 {
        let mut y = 0.;
        for coef in &self.enemy_coefs {
            y = coef + x * y;
        }
        y
    }
    pub fn eval_player_poly(&self, x: f64) -> f64 {
        let mut y = 0.;
        for coef in &self.player_coefs {
            y = coef + x * y;
        }
        y
    }

    /// Returns essentially an iterator that has points
    /// evenly spaced from the start of the poly to the end, but cut off at the
    /// `time` value.
    pub fn domain_range_time(&self, spacing: f64) -> LinSpace {
        // Lerp the end value between start_x and end_x using time as factor.
        LinSpace::new(
            self.start_x,
            self.start_x + (self.end_x - self.start_x) * self.time_taken / self.max_time,
            spacing,
        )
    }

    pub fn domain_range_limits(&self, spacing: f64) -> LinSpace {
        LinSpace::new(self.limits[0].x as f64, self.limits[1].x as f64, spacing)
    }
}

/// Assumes [`f64`] for now.
pub struct LinSpace {
    end: f64,
    current: f64,
    spacing: f64,
}

impl LinSpace {
    fn new(start: f64, end: f64, spacing: f64) -> Self {
        Self {
            end,
            current: start,
            spacing,
        }
    }
}

impl Iterator for LinSpace {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.spacing;
        if self.current > self.end {
            None
        } else {
            Some(self.current)
        }
    }
}
