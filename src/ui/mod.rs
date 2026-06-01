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
use iced::{Element, Length};

pub fn root<'a>(
    tracks: &'a [TrackConfig],
    pattern: &'a Pattern,
    length: BarLength,
    tempo: u16,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
    status: &'a str,
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let sidebar = track_sidebar::view(tracks, library);
    let main = column![
        header_controls(
            length,
            tempo,
            global_style,
            global_section,
            generation_options
        ),
        pattern_grid::view(tracks, pattern),
        status_bar(status, pattern)
    ]
    .spacing(16)
    .padding(18)
    .width(Length::Fill);

    row![sidebar, main].height(Length::Fill).into()
}

fn header_controls(
    length: BarLength,
    tempo: u16,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
) -> Element<'static, Message> {
    container(
        column![
            top_bar::view(length, tempo, global_style, global_section),
            musicality_controls::view(generation_options)
        ]
        .spacing(10)
        .width(Length::Fill),
    )
    .padding(12)
    .width(Length::Fill)
    .into()
}

fn status_bar<'a>(status: &'a str, pattern: &'a Pattern) -> Element<'a, Message> {
    let midi_hits = pattern
        .hits
        .iter()
        .filter(|hit| midi::note_for(hit.drum_type).is_some())
        .count();

    row![
        text(status).size(13),
        Space::with_width(Length::Fill),
        text(format!("{} bars / {} MIDI hits", pattern.bars, midi_hits)).size(13)
    ]
    .spacing(12)
    .into()
}
