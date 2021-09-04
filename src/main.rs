use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, CtxRef, FontFamily},
    EguiContext, EguiPlugin, EguiSettings,
};
use egui::plot::{Line, Plot, Value, Values};

mod level;
use level::Level;

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
        .init_resource::<DebugHelper>()
        .init_resource::<Level>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Initial screen
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

struct DebugHelper {
    color1: [u8; 3],
    color2: [u8; 3],
    color3: [u8; 3],
    color4: [u8; 3],
    color5: [u8; 3],
}

impl Default for DebugHelper {
    fn default() -> Self {
        Self {
            color1: [46, 86, 126],
            color2: [85, 91, 106],
            color3: [61, 104, 157],
            color4: [85, 91, 106],
            color5: [54, 55, 70],
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

fn ui_set_styles_and_fonts(ctx: &CtxRef, debug_helper: &ResMut<DebugHelper>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts
        .family_and_size
        .insert(egui::TextStyle::Button, (FontFamily::Monospace, 18.));
    fonts
        .family_and_size
        .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 32.));
    ctx.set_fonts(fonts);

    ctx.request_repaint();
    let mut style: egui::Style = (*ctx.style()).clone();
    style.visuals.override_text_color = Some(Color32::LIGHT_GRAY);
    let mut widget_styles = style.visuals.widgets.clone();
    widget_styles.active.bg_fill = Color32::from_rgb(
        debug_helper.color1[0],
        debug_helper.color1[1],
        debug_helper.color1[2],
    );
    widget_styles.inactive.bg_fill = Color32::from_rgb(
        debug_helper.color2[0],
        debug_helper.color2[1],
        debug_helper.color2[2],
    );
    widget_styles.hovered.bg_fill = Color32::from_rgb(
        debug_helper.color3[0],
        debug_helper.color3[1],
        debug_helper.color3[2],
    );
    widget_styles.open.bg_fill = Color32::from_rgb(
        debug_helper.color4[0],
        debug_helper.color4[1],
        debug_helper.color4[2],
    );
    widget_styles.noninteractive.bg_fill = Color32::from_rgb(
        debug_helper.color5[0],
        debug_helper.color5[1],
        debug_helper.color5[2],
    );
    style.visuals.widgets = widget_styles;
    ctx.set_style(style);
}

fn ui_main_menu(
    egui_ctx: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut debug_helper: ResMut<DebugHelper>,
) {
    let ctx = egui_ctx.ctx();
    ui_set_styles_and_fonts(ctx, &debug_helper);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(30.);
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

    egui::Window::new("Colors").show(ctx, |ui| {
        ui.color_edit_button_srgb(&mut debug_helper.color1);
        ui.color_edit_button_srgb(&mut debug_helper.color2);
        ui.color_edit_button_srgb(&mut debug_helper.color3);
        ui.color_edit_button_srgb(&mut debug_helper.color4);
        ui.color_edit_button_srgb(&mut debug_helper.color5);
    });
}

fn ui_level_menu(
    egui_ctx: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
    mut _ui_state: ResMut<DebugHelper>,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.vertical_centered(|ui| {
            if ui
                .add_sized(ui.available_size() / 8., egui::Button::new("Main Menu"))
                .clicked()
            {
                let _ = app_state.set(AppState::MainMenu);
            }
            ui.separator();
            ui.spacing_mut().item_spacing = egui::vec2(30., 30.);
            let widget_size = size_to_center_widgets(
                ui.available_size(),
                egui::vec2(3.0, 3.0),
                ui.spacing().item_spacing,
            );
            ui.add_space(widget_size.y);
            egui::Grid::new("Level Grid")
                .min_col_width(widget_size.x)
                .min_row_height(widget_size.y)
                .show(ui, |ui| {
                    ui.end_row();
                    for i in 0..3 {
                        ui.add_space(widget_size.x);
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
    mut level: ResMut<Level>,
    mut app_state: ResMut<State<AppState>>,
    time: Res<Time>,
) {
    let ctx = egui_ctx.ctx();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

    egui::CentralPanel::default().show(ctx, |ui| {
        level.time += time.delta_seconds_f64();
        let max = level.time.floor() as usize * 10;
        let path = (0..max).map(|i| {
            let x = i as f64 * 0.01;

            Value::new(x, level.eval_poly(x))
        });
        let last_x;
        if max > 0 {
            last_x = (max - 1) as f64 * 0.01;
        } else {
            last_x = 0.;
        }

        let last_y = level.eval_poly(last_x);
        let path = Line::new(Values::from_values_iter(path));
        let plot = Plot::new("rocket_path")
            .line(path)
            .allow_drag(false)
            .include_x(last_x)
            .include_y(last_y);
        ui.add(plot);

        ctx.request_repaint();
    });

    egui::Window::new("Window").show(egui_ctx.ctx(), |ui| {
        ui.label("Change the plot.");
        ui.add(egui::DragValue::new(&mut level.coefs[0]).prefix("a: "));
        ui.add(egui::DragValue::new(&mut level.coefs[1]).prefix("b: "));
        ui.add(egui::DragValue::new(&mut level.coefs[2]).prefix("c: "));
    });
}
