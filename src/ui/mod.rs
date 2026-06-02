mod constants;
mod musicality_controls;
mod pattern_grid;
mod top_bar;
mod track_sidebar;

use crate::app::Message;
use crate::midi;
use crate::models::{BarLength, GenerationOptions, Pattern, SongSection, Style, TrackConfig};
use crate::patterns::SourcePatternLibrary;
use iced::widget::{column, container, row, text, Space};
use iced::{Background, Border, Color, Element, Length, Theme};

pub fn root<'a>(
    tracks: &'a [TrackConfig],
    pattern: &'a Pattern,
    length: BarLength,
    tempo: u16,
    tempo_input: &'a str,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
    selected_track_index: usize,
    status: &'a str,
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let sidebar = track_sidebar::view(tracks, library, selected_track_index);
    let pattern_area = column![
        pattern_header(pattern, global_section),
        pattern_grid::view(tracks, pattern),
        status_bar(status, pattern, library)
    ]
    .spacing(14)
    .padding([0, 18])
    .width(Length::Fill);

    let content = column![
        header_controls(
            length,
            tempo,
            tempo_input,
            global_style,
            global_section,
            generation_options
        ),
        row![sidebar, pattern_area].height(Length::Fill)
    ]
    .spacing(18)
    .padding(18)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(app_background_style)
        .into()
}

fn header_controls<'a>(
    length: BarLength,
    tempo: u16,
    tempo_input: &'a str,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
) -> Element<'a, Message> {
    container(
        column![
            top_bar::view(),
            top_bar::settings_view(length, tempo, tempo_input, global_style, global_section),
            musicality_controls::view(generation_options)
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .padding(14)
    .width(Length::Fill)
    .style(raised_panel_style)
    .into()
}

fn pattern_header(pattern: &Pattern, section: SongSection) -> Element<'static, Message> {
    row![
        text(format!(
            "Pattern: {} · {} bars · {} hits",
            section,
            pattern.bars,
            pattern.hits.len()
        ))
        .size(16),
        Space::with_width(Length::Fill)
    ]
    .into()
}

fn status_bar<'a>(
    status: &'a str,
    pattern: &'a Pattern,
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let midi_hits = pattern
        .hits
        .iter()
        .filter(|hit| midi::note_for(hit.drum_type).is_some())
        .count();

    row![
        text(status).size(13),
        Space::with_width(Length::Fill),
        text(format!(
            "Generated {} bars · {} hits · {} source patterns · {} MIDI hits",
            pattern.bars,
            pattern.hits.len(),
            library.pattern_count(),
            midi_hits
        ))
        .size(13)
    ]
    .spacing(12)
    .into()
}

pub fn theme() -> Theme {
    Theme::custom(
        String::from("DrumViper"),
        iced::theme::Palette {
            background: color(0x11, 0x13, 0x18),
            text: color(0xe8, 0xea, 0xed),
            primary: color(0x6c, 0x8c, 0xff),
            success: color(0x40, 0xd6, 0xb0),
            danger: color(0xff, 0x75, 0x75),
        },
    )
}

fn app_background_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color(0x11, 0x13, 0x18))),
        text_color: Some(color(0xe8, 0xea, 0xed)),
        ..container::Style::default()
    }
}

fn raised_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color(0x20, 0x24, 0x2d))),
        text_color: Some(color(0xe8, 0xea, 0xed)),
        border: Border {
            color: color(0x30, 0x36, 0x41),
            width: 1.0,
            radius: 7.0.into(),
        },
        ..container::Style::default()
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}
