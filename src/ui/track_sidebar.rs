use crate::app::Message;
use crate::models::{DrumType, GenerationOptions, Style, TrackConfig};
use crate::patterns::SourcePatternLibrary;
use crate::ui::constants::TRACK_SIDEBAR_WIDTH;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, slider, text, Space,
};
use iced::{Alignment, Element, Length};

pub fn view<'a>(
    tracks: &'a [TrackConfig],
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let mut controls = column![text("Tracks").size(24)].spacing(12).padding(14);

    for (index, track) in tracks.iter().enumerate() {
        controls = controls.push(track_controls(index, track, library));
    }

    container(scrollable(controls))
        .width(Length::Fixed(TRACK_SIDEBAR_WIDTH))
        .height(Length::Fill)
        .into()
}

fn track_controls<'a>(
    index: usize,
    track: &'a TrackConfig,
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let instrument_count = library.instrument_count(track.drum_type);

    let content = column![
        row![
            checkbox(format!("Track {}", index + 1), track.enabled)
                .on_toggle(move |enabled| Message::TrackEnabledChanged(index, enabled)),
            Space::with_width(Length::Fill),
            checkbox("Lock", track.locked)
                .on_toggle(move |locked| Message::TrackLockedChanged(index, locked)),
            button("Randomize").on_press(Message::RandomizeTrack(index))
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        field(
            "Drum",
            pick_list(
                Vec::from(DrumType::ALL),
                Some(track.drum_type),
                move |drum_type| Message::TrackDrumChanged(index, drum_type)
            )
            .width(Length::Fill)
            .into()
        ),
        field(
            "Style",
            pick_list(Vec::from(Style::ALL), Some(track.style), move |style| {
                Message::TrackStyleChanged(index, style)
            })
            .width(Length::Fill)
            .into()
        ),
        row![
            text(format!("Activity: {}%", track.probability)).width(Length::Fixed(112.0)),
            slider(
                GenerationOptions::MIN..=GenerationOptions::MAX,
                track.probability,
                move |probability| Message::TrackProbabilityChanged(index, probability)
            )
            .width(Length::Fill)
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        text(format!("{} source rows", instrument_count)).size(12)
    ]
    .spacing(8);

    container(content).padding(10).width(Length::Fill).into()
}

fn field<'a>(label: &'static str, control: Element<'a, Message>) -> Element<'a, Message> {
    row![text(label).width(Length::Fixed(64.0)), control]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
}
