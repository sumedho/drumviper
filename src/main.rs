mod app;
mod generator;
mod midi;
mod models;
mod patterns;
mod style_rules;
mod ui;

fn main() -> iced::Result {
    iced::application("DrumViper", app::DrumViper::update, app::DrumViper::view)
        .theme(app::DrumViper::theme)
        .window_size((1_600.0, 900.0))
        .run_with(app::DrumViper::new)
}
