use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod grid;
use grid::Grid;

/// Used to help identify our main camera
struct MainCamera;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(test_movement_with_mouse.system())
        .run();
}

fn setup(windows: Res<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let grid = Grid::square_grid(width * 0.8, height * 0.8, 100.);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &grid,
            ShapeColors::new(Color::BLACK),
            DrawMode::Stroke(StrokeOptions::default().with_line_width(5.0)),
            Transform::default(),
        ))
        .insert(grid);
}

fn test_movement_with_mouse(
    mut commands: Commands,
    windows: Res<Windows>,
    mut mouse_wheel: EventReader<MouseWheel>,
    query: Query<(Entity, &Grid)>,
    camera_q: Query<&Transform, With<MainCamera>>,
) {
    if let Ok((entity, grid)) = query.single() {
        let window = windows.get_primary().unwrap();
        if let Some(cursor_pos) = window.cursor_position() {
            let mut new_size = (grid.cell_size.x + grid.cell_size.y) / 2.;
            for mouse_wheel in mouse_wheel.iter() {
                new_size -= (mouse_wheel.x + mouse_wheel.y) / 2.;
            }
            let camera_transform = camera_q.single().unwrap();
            let new_grid = Grid::square_grid_at(
                grid.width,
                grid.height,
                new_size,
                cursor_to_world(cursor_pos, window, camera_transform),
            );
            commands.entity(entity).despawn();
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &new_grid,
                    ShapeColors::new(Color::BLACK),
                    DrawMode::Stroke(StrokeOptions::default().with_line_width(5.0)),
                    Transform::default(),
                ))
                .insert(new_grid);
        }
    }
}

/// Converts the cursor position to world coordinates. Returns a [`Vec2`] containing
/// the `x` and `y` coordinates of the cursor in world coordinates.
fn cursor_to_world(cursor_pos: Vec2, window: &Window, camera_transform: &Transform) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = cursor_pos - size / 2.0;

    // apply the camera transform
    let pos_world = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
    Vec2::new(pos_world.x, pos_world.y)
}
