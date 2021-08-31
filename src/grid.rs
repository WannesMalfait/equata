use bevy::math::Vec2;
use bevy_prototype_lyon::geometry::Geometry;
use lyon_tessellation::{
    geom::{LineSegment, Point},
    path::{path::Builder, traits::PathBuilder},
};

/// A grid consisting of a number of vertical and horizontal line segments.
/// There is always a vertical and a horizontal line that passes through the
/// center of the grid. The distance between the vertical and horizontal segments
/// is determined by `cell_size`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub width: f32,
    pub height: f32,
    pub cell_size: Vec2,
    pub center: Vec2,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 100.,
            height: 100.,
            cell_size: Vec2::new(10., 10.),
            center: Vec2::ZERO,
        }
    }
}

impl Grid {
    /// Creates a [`Grid`] with the given `width` and `height`, and such that the size of the
    /// cells matches `size`.
    pub fn square_grid(width: f32, height: f32, size: f32) -> Self {
        Self::square_grid_at(width, height, size, Vec2::ZERO)
    }

    /// Creates a [`Grid`] with the given `width`, `height` and `center`, and such that the
    /// size of the cells matches `size`.
    pub fn square_grid_at(width: f32, height: f32, size: f32, center: Vec2) -> Self {
        Self {
            width,
            height,
            cell_size: Vec2::new(size, size),
            center,
        }
    }
}

impl Geometry for Grid {
    fn add_geometry(&self, b: &mut Builder) {
        let half_numx = (self.width / self.cell_size.x / 2.).floor() as i32;
        let half_numy = (self.height / self.cell_size.y / 2.).floor() as i32;
        for i in -half_numx..(half_numx + 1) {
            let x = self.center.x + i as f32 * self.cell_size.x;
            let line = LineSegment {
                from: Point::new(x, self.center.y - self.height / 2.),
                to: Point::new(x, self.center.y + self.height / 2.),
            };
            b.add_line_segment(&line);
        }
        for i in -half_numy..(half_numy + 1) {
            let y = self.center.y + i as f32 * self.cell_size.y;
            let line = LineSegment {
                from: Point::new(self.center.x - self.width / 2., y),
                to: Point::new(self.center.x + self.width / 2., y),
            };
            b.add_line_segment(&line);
        }
    }
}
