use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use egui::plot::{Line, Plot, Value, Values};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    LevelMenu,
    InGame,
    Paused,
}
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 100.0)))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::MainMenu)
        // Always running
        .add_system(update_ui_scale_factor.system())
        // Main menu
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(ui_main_menu.system()))
        // Level menu
        .add_system_set(
            SystemSet::on_update(AppState::LevelMenu).with_system(ui_level_menu.system()),
        )
        // In Game
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(ui_ingame.system()))
        // Paused
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(ui_pause_menu.system()))
        .run();
}

struct UiState {
    value: f64,
    size: f32,
    spacing: egui::Vec2,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            value: 1.0,
            size: 5.0,
            spacing: egui::vec2(10., 20.),
        }
    }
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

/// Returns a size such that the widgets will appear to be nicely centered.
fn size_to_center_widgets(
    total_size: egui::Vec2,
    num_buttons: egui::Vec2,
    spacing: egui::Vec2,
) -> egui::Vec2 {
    // The area filled up by the buttons is the total size minus the amount of space between the buttons.
    // We then divide this by the number of buttons + 2 such that there is space for 1 invisible button on
    // both sides.
    (total_size - spacing * (num_buttons - egui::vec2(1., 1.))) / (num_buttons + egui::vec2(2., 2.))
}

fn ui_main_menu(
    egui_ctx: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut ui_state: ResMut<UiState>,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Equata");
            ui.spacing_mut().item_spacing = egui::vec2(30., 30.);
            let widget_size = size_to_center_widgets(
                ui.available_size(),
                egui::vec2(1.0, 3.0),
                ui.spacing().item_spacing,
            );
            ui.add_space(widget_size.y);
            if ui
                .add_sized(widget_size, egui::Button::new("Level"))
                .clicked()
            {
                let _ = app_state.set(AppState::LevelMenu);
            }
            ui.add_sized(widget_size, egui::Button::new("Help"));
            if ui
                .add_sized(widget_size, egui::Button::new("Quit"))
                .clicked()
            {
                std::process::exit(0);
            }
        });
    });

    egui::Window::new("Edit").show(egui_ctx.ctx(), |ui| {
        ui.add(egui::Slider::new(&mut ui_state.spacing.y, 0.0..=100.0).text("Y spacing"));
        ui.add(egui::Slider::new(&mut ui_state.size, 0.1..=10.0).text("Size Divisor"));
    });
}

fn ui_level_menu(
    egui_ctx: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut _ui_state: ResMut<UiState>,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.vertical_centered(|ui| {
            if ui.button("Main Menu").clicked() {
                let _ = app_state.set(AppState::MainMenu);
            }
            ui.separator();
            let widget_size = size_to_center_widgets(
                ui.available_size(),
                egui::vec2(3.0, 3.0),
                ui.spacing().item_spacing,
            );
            egui::Grid::new("Level Grid")
                .striped(true)
                .min_col_width(widget_size.x)
                .min_row_height(widget_size.y)
                .show(ui, |ui| {
                    ui.end_row();
                    for i in 0..3 {
                        for j in 0..3 {
                            if ui
                                .add_sized(widget_size, egui::Button::new(format!("{}, {}", i, j)))
                                .clicked()
                            {
                                let _ = app_state.set(AppState::InGame);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    });
}

fn ui_pause_menu(egui_ctx: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.vertical_centered(|ui| {
            ui.label("Resume");
            if ui
                .button("Main Menu")
                .on_hover_text("Go to the main menu and stop the current game in progress.")
                .clicked()
            {
                let _ = app_state.set(AppState::MainMenu);
            }
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });
    });
}

fn ui_ingame(
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "Options", |ui| {
                if ui.button("Main Menu").clicked() {
                    let _ = app_state.set(AppState::MainMenu);
                }
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.heading("Plot");
        let sin = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            Value::new(x, x.sin() * ui_state.value)
        });
        let line = Line::new(Values::from_values_iter(sin));
        ui.add(Plot::new("my_plot").line(line).allow_zoom(true));
    });

    egui::Window::new("Window").show(egui_ctx.ctx(), |ui| {
        ui.label("Change the plot.");
        ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            ui_state.value += 1.0;
        }
    });
}
