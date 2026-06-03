use crate::app::Message;
use crate::models::{BarLength, SongSection, Style};
use iced::widget::{button, container, mouse_area, pick_list, row, text, text_input, Space};
use iced::{mouse, Alignment, Background, Border, Color, Element, Length, Theme};

pub fn view() -> Element<'static, Message> {
    row![
        text("DrumViper").size(30),
        Space::with_width(Length::Fill),
        drag_midi_control(),
        button("Generate")
            .padding([9, 14])
            .style(secondary_button_style)
            .on_press(Message::RandomizeAll),
        button("Export MIDI")
            .padding([9, 16])
            .style(primary_button_style)
            .on_press(Message::ExportMidi)
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn drag_midi_control() -> Element<'static, Message> {
    mouse_area(
        container(text("Drag MIDI"))
            .padding([9, 14])
            .style(subtle_container_style),
    )
    .on_press(Message::DragMidi)
    .interaction(mouse::Interaction::Grab)
    .into()
}

pub fn settings_view<'a>(
    length: BarLength,
    tempo: u16,
    tempo_input: &'a str,
    global_style: Style,
    global_section: SongSection,
) -> Element<'a, Message> {
    row![
        labeled_pick_list(
            "Style",
            Style::all(),
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
        row![
            text("Tempo"),
            text_input("", tempo_input)
                .on_input(Message::TempoInputChanged)
                .width(Length::Fixed(70.0))
                .padding([7, 9]),
            text("BPM")
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        Space::with_width(Length::Fill),
        text(format!("Current: {tempo} BPM")).width(Length::Fixed(120.0)),
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

fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        color(0x4a, 0xe4, 0xc0)
    } else {
        color(0x40, 0xd6, 0xb0)
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: color(0x11, 0x13, 0x18),
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        color(0x7d, 0x9b, 0xff)
    } else {
        color(0x6c, 0x8c, 0xff)
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: color(0xe8, 0xea, 0xed),
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

fn subtle_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color(0x18, 0x1b, 0x22))),
        text_color: Some(color(0xe8, 0xea, 0xed)),
        border: Border {
            color: color(0x30, 0x36, 0x41),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}
