mod pattern_grid;

use crate::app::Message;
use crate::midi;
use crate::models::{
    BarLength, DrumType, GenerationOptions, Pattern, SongSection, Style, TrackConfig,
};
use crate::patterns::SourcePatternLibrary;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, slider, text, Space,
};
use iced::{Alignment, Element, Length};

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
    let sidebar = track_sidebar(tracks, library);
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

fn track_sidebar<'a>(
    tracks: &'a [TrackConfig],
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let mut controls = column![text("Tracks").size(24)].spacing(12).padding(14);

    for (index, track) in tracks.iter().enumerate() {
        controls = controls.push(track_controls(index, track, library));
    }

    container(scrollable(controls))
        .width(Length::Fixed(340.0))
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
        pick_list(
            Vec::from(DrumType::ALL),
            Some(track.drum_type),
            move |drum_type| Message::TrackDrumChanged(index, drum_type)
        )
        .width(Length::Fill),
        pick_list(Vec::from(Style::ALL), Some(track.style), move |style| {
            Message::TrackStyleChanged(index, style)
        })
        .width(Length::Fill),
        row![
            text(format!("Amount: {}", track.probability)).width(Length::Fixed(96.0)),
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

fn header_controls(
    length: BarLength,
    tempo: u16,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
) -> Element<'static, Message> {
    let tempo_slider = slider(60..=180, tempo, Message::TempoChanged);

    let control_row = row![
        text("DrumViper").size(30),
        Space::with_width(Length::Fixed(18.0)),
        text("Style"),
        pick_list(
            Vec::from(Style::ALL),
            Some(global_style),
            Message::GlobalStyleChanged
        )
        .width(Length::Fixed(185.0)),
        text("Section"),
        pick_list(
            Vec::from(SongSection::ALL),
            Some(global_section),
            Message::GlobalSectionChanged
        )
        .width(Length::Fixed(150.0)),
        text("Length"),
        pick_list(
            Vec::from(BarLength::ALL),
            Some(length),
            Message::LengthChanged
        )
        .width(Length::Fixed(140.0)),
        Space::with_width(Length::Fixed(18.0)),
        text(format!("Tempo: {} BPM", tempo)).width(Length::Fixed(130.0)),
        tempo_slider.width(Length::Fill),
        button("Randomize").on_press(Message::RandomizeAll),
        button("Export MIDI").on_press(Message::ExportMidi)
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let musicality_row = row![
        slider_control(
            "Density",
            generation_options.density,
            Message::DensityChanged
        ),
        slider_control(
            "Complexity",
            generation_options.complexity,
            Message::ComplexityChanged
        ),
        slider_control(
            "Fill",
            generation_options.fill_amount,
            Message::FillAmountChanged
        ),
        slider_control("Groove", generation_options.groove, Message::GrooveChanged),
        slider_control(
            "Humanize",
            generation_options.humanize,
            Message::HumanizeChanged
        )
    ]
    .spacing(18)
    .align_y(Alignment::Center);

    container(
        column![control_row, musicality_row]
            .spacing(10)
            .width(Length::Fill),
    )
    .padding(12)
    .width(Length::Fill)
    .into()
}

fn slider_control(
    label: &'static str,
    value: u8,
    on_change: fn(u8) -> Message,
) -> Element<'static, Message> {
    let display = if label == "Density" {
        format!(
            "{label}: {} {}",
            GenerationOptions {
                density: value,
                ..GenerationOptions::default()
            }
            .density_label(),
            value
        )
    } else {
        format!("{label}: {value}")
    };

    row![
        text(display).width(Length::Fixed(132.0)),
        slider(
            GenerationOptions::MIN..=GenerationOptions::MAX,
            value,
            on_change
        )
        .width(Length::Fill)
    ]
    .spacing(8)
    .align_y(Alignment::Center)
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
