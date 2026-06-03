mod app;
mod drag_export;
mod generator;
mod microtiming;
mod midi;
mod models;
mod patterns;
mod source_patterns;
mod style_rules;
mod ui;

fn main() -> iced::Result {
    iced::application("DrumViper", app::DrumViper::update, app::DrumViper::view)
        .theme(app::DrumViper::theme)
        .subscription(app::DrumViper::subscription)
        .window_size((1_600.0, 900.0))
        .run_with(app::DrumViper::new)
}
