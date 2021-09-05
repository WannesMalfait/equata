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
    /// The player has won this level (used for UI)
    pub won: bool,
    /// The player has lost i.e. time_taken > max_time
    pub lost: bool,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            enemy_coefs: vec![-1.0, 0.0, 1.0],
            player_coefs: vec![1.0; 3],
            limits: [Vec2::new(-2., -1.), Vec2::new(2., 2.)],
            max_time: 20.0,
            time_taken: 0.0,
            start_x: -1.0,
            end_x: 1.0,
            won: false,
            lost: false,
        }
    }
}
impl Level {
    fn eval_poly(x: f64, coefs: &Vec<f64>) -> f64 {
        let mut y = 0.;
        for coef in coefs {
            y = coef + x * y;
        }
        y
    }

    fn min_max(start: f64, end: f64, coefs: &Vec<f64>) -> [f64; 2] {
        let mut min = Level::eval_poly(start, coefs);
        let mut max = min;
        for x in LinSpace::new(start, end, 0.01) {
            let y = Level::eval_poly(x, coefs);
            if y < min {
                min = y;
            } else if y > max {
                max = y;
            }
        }
        [min, max]
    }

    fn bin_search_root(mut start: f64, mut end: f64, coefs: &Vec<f64>) -> f64 {
        let mut y_val = Level::eval_poly(start, coefs);
        let start_is_negative = y_val <= 0.0;
        let mut mid = start;
        let mut count_max = 20;
        while y_val.abs() > 1e-10 && count_max > 0 {
            mid = (start - end) / 2.;
            y_val = Level::eval_poly(mid, coefs);
            if start_is_negative == (y_val <= 0.0) {
                start = mid;
            } else {
                end = mid;
            }
            count_max -= 1;
        }
        mid
    }

    fn get_roots(coefs: &Vec<f64>) -> Vec<f64> {
        let mut roots = Vec::new();
        if coefs.len() < 1 {
            return roots;
        }
        let mut y_is_negative = Level::eval_poly(-10., coefs) <= 0.0;
        let mut prev_x = 0.0;
        for x in LinSpace::new(-10., 10., 0.1) {
            let y = Level::eval_poly(x, coefs);
            if y_is_negative != (y <= 0.0) {
                // Changed sign so there is a root.
                roots.push(Level::bin_search_root(prev_x.clone(), x.clone(), coefs));
            }
            y_is_negative = y <= 0.0;
            prev_x = x;
        }
        roots
    }

    pub fn new(enemy_coefs: impl IntoIterator<Item = f64>, max_time: f64) -> Result<Self, String> {
        let enemy_coefs = enemy_coefs.into_iter().collect::<Vec<f64>>();
        let len = enemy_coefs.len();
        let roots = Level::get_roots(&enemy_coefs);
        if roots.len() < 2 {
            return Err("Needs at least 2 roots.".to_string());
        }
        let start_x = roots[0];
        let end_x = roots[1];
        let min_max = Level::min_max(start_x, end_x, &enemy_coefs);
        println!("{},{}, {:?}", start_x, end_x, min_max);
        Ok(Self {
            enemy_coefs,
            player_coefs: vec![1.0; len],
            limits: [
                Vec2::new(start_x as f32 - 1.0, min_max[0] as f32 - 1.0),
                Vec2::new(end_x as f32 + 1.0, min_max[1] as f32 + 1.0),
            ],
            max_time,
            start_x,
            end_x,
            ..Default::default()
        })
    }

    pub fn restart(&mut self) {
        self.time_taken = 0.;
        self.player_coefs = vec![1.0; self.player_coefs.len()];
        self.won = false;
        self.lost = false;
    }

    pub fn check_won(&mut self) -> bool {
        let mut res = true;
        for i in 0..self.enemy_coefs.len() {
            if (self.enemy_coefs[i] - self.player_coefs[i]).abs() > 0.01 {
                res = false;
                break;
            }
        }
        self.won = res;
        res
    }

    pub fn eval_enemy_poly(&self, x: f64) -> f64 {
        Level::eval_poly(x, &self.enemy_coefs)
    }
    pub fn eval_player_poly(&self, x: f64) -> f64 {
        Level::eval_poly(x, &self.player_coefs)
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
        let temp = self.current;
        self.current += self.spacing;
        if temp > self.end {
            None
        } else {
            Some(temp)
        }
    }
}
