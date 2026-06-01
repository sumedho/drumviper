use crate::app::Message;
use crate::models::{BarLength, SongSection, Style, MAX_TEMPO_BPM, MIN_TEMPO_BPM};
use iced::widget::{button, pick_list, row, slider, text, Space};
use iced::{Alignment, Element, Length};

pub fn view(
    length: BarLength,
    tempo: u16,
    global_style: Style,
    global_section: SongSection,
) -> Element<'static, Message> {
    row![
        text("DrumViper").size(30),
        Space::with_width(Length::Fixed(18.0)),
        labeled_pick_list(
            "Style",
            Vec::from(Style::ALL),
            Some(global_style),
            Message::GlobalStyleChanged,
            185.0
        ),
        labeled_pick_list(
            "Section",
            Vec::from(SongSection::ALL),
            Some(global_section),
            Message::GlobalSectionChanged,
            150.0
        ),
        labeled_pick_list(
            "Length",
            Vec::from(BarLength::ALL),
            Some(length),
            Message::LengthChanged,
            140.0
        ),
        Space::with_width(Length::Fixed(12.0)),
        text(format!("Tempo: {} BPM", tempo)).width(Length::Fixed(130.0)),
        slider(MIN_TEMPO_BPM..=MAX_TEMPO_BPM, tempo, Message::TempoChanged).width(Length::Fill),
        button("Randomize").on_press(Message::RandomizeAll),
        button("Export MIDI").on_press(Message::ExportMidi)
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn labeled_pick_list<T: Clone + ToString + PartialEq + 'static>(
    label: &'static str,
    options: Vec<T>,
    selected: Option<T>,
    on_selected: fn(T) -> Message,
    width: f32,
) -> Element<'static, Message> {
    row![
        text(label),
        pick_list(options, selected, on_selected).width(Length::Fixed(width))
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}
