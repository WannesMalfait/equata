use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self,
        plot::{Legend, Points},
        Color32, CtxRef, FontFamily, Frame,
    },
    EguiContext, EguiPlugin, EguiSettings,
};
use egui::plot::{Line, Plot, Value, Values};

mod level;
use level::Level;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    LevelMenu,
    About,
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
        .add_system(handle_keys.system())
        // Main menu
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(ui_main_menu.system()))
        // Help screen
        .add_system_set(SystemSet::on_update(AppState::About).with_system(ui_about_screen.system()))
        // Level menu
        .add_system_set(
            SystemSet::on_update(AppState::LevelMenu).with_system(ui_level_menu.system()),
        )
        // In Game
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(ui_ingame.system()))
        // Paused still has same function, but behaves differently.
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(ui_ingame.system()))
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
            color2: [66, 92, 121],
            color3: [74, 119, 157],
            color4: [85, 91, 106],
            color5: [59, 59, 74],
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

fn handle_keys(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if app_state.current() == &AppState::InGame
        && (keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Escape))
    {
        let _ = app_state.set(AppState::Paused);
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
        .insert(egui::TextStyle::Body, (FontFamily::Proportional, 16.));
    fonts
        .family_and_size
        .insert(egui::TextStyle::Small, (FontFamily::Proportional, 14.));
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
                .on_hover_text("Select a level to play.")
                .clicked()
            {
                let _ = app_state.set(AppState::LevelMenu);
            }
            if ui
                .add_sized(widget_size, egui::Button::new("About"))
                .on_hover_text("Info about the game and author.")
                .on_hover_text("If you need help to understand the game, this is the place to go.")
                .clicked()
            {
                let _ = app_state.set(AppState::About);
            }
            if ui
                .add_sized(widget_size, egui::Button::new("Quit"))
                .on_hover_text("Exit the game.")
                .clicked()
            {
                std::process::exit(0);
            }
        });
    });

    egui::Window::new("Colors").show(ctx, |ui| {
        ui.label("Active");
        ui.color_edit_button_srgb(&mut debug_helper.color1);
        ui.label("Inactive");
        ui.color_edit_button_srgb(&mut debug_helper.color2);
        ui.label("Hovered");
        ui.color_edit_button_srgb(&mut debug_helper.color3);
        ui.label("Open");
        ui.color_edit_button_srgb(&mut debug_helper.color4);
        ui.label("NonInteractive");
        ui.color_edit_button_srgb(&mut debug_helper.color5);
    });
}

fn ui_about_screen(egui_ctx: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        ui.vertical_centered(|ui| {
            if ui
                .add_sized(ui.available_size() / 8., egui::Button::new("Main Menu"))
                .clicked()
            {
                let _ = app_state.set(AppState::MainMenu);
            }
            ui.separator();
            ui.heading("Equata");
            ui.label("An enemy has launched a missile!");
            ui.label("To stop the missile from hitting the town, you need to predict its path.");
            ui.label("Use the control panel to make a prediction about the path's FUTURE.");
            ui.label("Click 'Confirm' when you are confident of your prediction. But be careful, any mistakes will take away a second of your precious time!");
            ui.label("Press 'SPACE' or 'ESCAPE' at any time to pause.");
            ui.separator();
            ui.heading("About Equata");
            ui.label("Equata was made for the OLC 2021 Code Jam.");
            ui.label("Equata is written in rust using the Bevy game engine and egui.");
            ui.hyperlink_to("Source Code on GitHub", "https://github.com/WannesMalfait/equata");
        });
    });
}

fn ui_level_menu(egui_ctx: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
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
            let difficulties = ["Easy", "Medium", "Hard"];
            egui::Grid::new("Level Grid")
                .min_col_width(widget_size.x)
                .min_row_height(widget_size.y)
                .show(ui, |ui| {
                    ui.end_row();
                    for i in 0..3 {
                        ui.add_space(widget_size.x);
                        for j in 0..3 {
                            if ui
                                .add_sized(
                                    widget_size,
                                    egui::Button::new(format!(
                                        "Level {} {}",
                                        i + 1,
                                        difficulties[j]
                                    )),
                                )
                                .clicked()
                            {
                                // TODO: Set the level
                                let _ = app_state.set(AppState::InGame);
                            }
                        }
                        ui.end_row();
                    }
                });
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

    let available_rect = ctx.available_rect();
    let available_width = available_rect.width();
    let available_height = available_rect.height();

    let playing = app_state.current() == &AppState::InGame && !level.won && !level.lost;
    // Game is displayed here.
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.set_enabled(playing);
        if playing {
            level.time_taken += time.delta_seconds_f64();
            if level.time_taken >= level.max_time {
                level.lost = true;
            }
        }
        ui.label(format!("Time left: {}s", level.max_time - level.time_taken));
        // Draw the background even when paused

        // Calculate the paths for the player and enemy
        let enemy_path = Line::new(Values::from_values_iter(
            level
                .domain_range_time(0.01)
                .map(|x| Value::new(x, level.eval_enemy_poly(x))),
        ))
        .name("Enemy Path")
        .color(Color32::RED)
        .width(2.5);

        let player_path = Points::new(Values::from_values_iter(
            level
                // Bigger spacing because it's just points.
                .domain_range_limits(0.025)
                .map(|x| Value::new(x, level.eval_player_poly(x))),
        ))
        .name("Prediction")
        .color(Color32::GREEN)
        .radius(2.5);

        let mut plot = Plot::new("rocket_paths")
            .line(enemy_path)
            .points(player_path)
            .allow_drag(false)
            .legend(Legend {
                background_alpha: 0.5,
                ..Default::default()
            });
        for limit in level.limits {
            plot = plot.include_x(limit.x);
            plot = plot.include_y(limit.y);
        }
        ui.add(plot);

        ctx.request_repaint();
    });

    // Control window
    let mut frame = Frame::window(&ctx.style());
    frame.fill =
        Color32::from_rgba_premultiplied(frame.fill.r(), frame.fill.g(), frame.fill.b(), 100);
    egui::Window::new("Controls").frame(frame).show(ctx, |ui| {
        ui.set_enabled(playing);
        ui.label("Change the path to match that of your enemy using the controls.");
        let mut equation = String::from("");
        for i in 0..level.enemy_coefs.len() {
            equation += &char::from_u32(97 + i as u32).unwrap().to_string();
            match level.enemy_coefs.len() - 1 - i {
                0 => continue,
                1 => equation += "x + ",
                n => equation += &format!("x^{} + ", n),
            }
        }
        ui.label(format!("Path: {}", equation));
        for i in 0..level.enemy_coefs.len() {
            ui.add(
                egui::DragValue::new(&mut level.player_coefs[i])
                    .prefix(format!("{}: ", char::from_u32(97 + i as u32).unwrap())),
            );
        }
        if ui
            .button("Confirm")
            .on_hover_text("Confirm path prediction.")
            .on_hover_text("Incorrect prediction will result in a time penalty.")
            .clicked()
        {
            if !level.check_won() {
                level.time_taken += 1.0;
            }
        }
    });

    // Pause Window
    let mut frame = Frame::window(&ctx.style());
    frame.margin = egui::vec2(50., 20.);
    frame.fill =
        Color32::from_rgba_premultiplied(frame.fill.r(), frame.fill.g(), frame.fill.b(), 100);
    if app_state.current() == &AppState::Paused {
        egui::Window::new("Paused")
            .frame(frame)
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(10.);
                    ui.spacing_mut().item_spacing = egui::vec2(30., 30.);
                    let widget_size = size_to_center_widgets(
                        egui::vec2(available_width, available_height),
                        egui::vec2(1.0, 4.0),
                        ui.spacing().item_spacing,
                    );
                    if ui
                        .add_sized(widget_size, egui::Button::new("Resume"))
                        .on_hover_text("Resume the game where it was paused.")
                        .clicked()
                    {
                        let _ = app_state.set(AppState::InGame);
                    }
                    if ui
                        .add_sized(widget_size, egui::Button::new("Restart"))
                        .on_hover_text("Restart the level. Any progress will be lost.")
                        .clicked()
                    {
                        level.restart();
                        let _ = app_state.set(AppState::InGame);
                    }
                    if ui
                        .add_sized(widget_size, egui::Button::new("Main Menu"))
                        .clicked()
                    {
                        let _ = app_state.set(AppState::MainMenu);
                    }
                    if ui
                        .add_sized(widget_size, egui::Button::new("Quit"))
                        .on_hover_text("Exit the game.")
                        .clicked()
                    {
                        std::process::exit(0);
                    }
                });
            });
    }

    if !level.won && !level.lost || app_state.current() == &AppState::Paused {
        return;
    }
    // Win-lose window
    let mut frame = Frame::window(&ctx.style());
    frame.margin = egui::vec2(50., 20.);
    if level.won {
        // Greenish
        frame.fill = Color32::from_rgba_premultiplied(20, 80, 30, 150);
    } else {
        // Redish
        frame.fill = Color32::from_rgba_premultiplied(90, 30, 20, 150);
    }
    egui::Window::new("Game Over")
        .frame(frame)
        .title_bar(false)
        .resizable(false)
        // .fixed_size(egui::vec2(available_width * 0.9, available_height * 0.6))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(egui_ctx.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                if level.won {
                    ui.heading("You win!");
                } else {
                    ui.heading("You lose!");
                }
            });
            ui.vertical(|ui| {
                ui.add_space(20.);
                ui.spacing_mut().item_spacing = egui::vec2(30., 30.);
                let widget_size = size_to_center_widgets(
                    egui::vec2(available_width, available_height),
                    egui::vec2(1.0, 5.0),
                    ui.spacing().item_spacing,
                );
                if level.lost {
                    if ui
                        .add_sized(widget_size, egui::Button::new("Restart"))
                        .on_hover_text("Restart the level. Any progress will be lost.")
                        .clicked()
                    {
                        level.restart();
                        let _ = app_state.set(AppState::InGame);
                    }
                }

                if ui
                    .add_sized(widget_size, egui::Button::new("Level Menu"))
                    .on_hover_text("Select a level to play.")
                    .clicked()
                {
                    let _ = app_state.set(AppState::LevelMenu);
                }
                if ui
                    .add_sized(widget_size, egui::Button::new("Main Menu"))
                    .clicked()
                {
                    let _ = app_state.set(AppState::MainMenu);
                }
                if ui
                    .add_sized(widget_size, egui::Button::new("Quit"))
                    .on_hover_text("Exit the game.")
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });
}
