use crate::app::Message;
use crate::models::{DrumType, GenerationOptions, Style, TrackConfig};
use crate::patterns::SourcePatternLibrary;
use crate::ui::constants::TRACK_SIDEBAR_WIDTH;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, slider, text, Space,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

pub fn view<'a>(
    tracks: &'a [TrackConfig],
    library: &'a SourcePatternLibrary,
    selected_track_index: usize,
) -> Element<'a, Message> {
    let mut list = column![sidebar_header()].spacing(12).padding(14);

    for (index, track) in tracks.iter().enumerate() {
        list = list.push(track_list_item(index, track, index == selected_track_index));
    }

    let selected = tracks.get(selected_track_index);
    let inspector =
        selected.map(|track| selected_track_inspector(selected_track_index, track, library));

    let mut content = column![scrollable(list).height(Length::Fill)].spacing(12);
    if let Some(inspector) = inspector {
        content = content.push(inspector);
    }

    container(content.padding(8))
        .width(Length::Fixed(TRACK_SIDEBAR_WIDTH))
        .height(Length::Fill)
        .style(panel_style)
        .into()
}

fn sidebar_header() -> Element<'static, Message> {
    row![
        text("Tracks").size(24),
        Space::with_width(Length::Fill),
        text("Presence").size(12).style(secondary_text)
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn track_list_item(index: usize, track: &TrackConfig, selected: bool) -> Element<'_, Message> {
    let lock = if track.locked { "locked" } else { "unlocked" };
    let enabled_marker = if track.enabled { "on" } else { "off" };

    let content = row![
        checkbox(format!("{}", index + 1), track.enabled)
            .on_toggle(move |enabled| Message::TrackEnabledChanged(index, enabled)),
        text(track.drum_type.to_string()).width(Length::Fill),
        text(format!("{}%", track.probability))
            .width(Length::Fixed(48.0))
            .style(secondary_text),
        text(enabled_marker)
            .size(12)
            .width(Length::Fixed(28.0))
            .style(if track.enabled {
                success_text
            } else {
                secondary_text
            }),
        text(lock)
            .size(12)
            .width(Length::Fixed(64.0))
            .style(secondary_text),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    button(content)
        .width(Length::Fill)
        .padding([9, 10])
        .style(move |_theme, status| track_button_style(selected, status))
        .on_press(Message::TrackSelected(index))
        .into()
}

fn selected_track_inspector<'a>(
    index: usize,
    track: &'a TrackConfig,
    library: &'a SourcePatternLibrary,
) -> Element<'a, Message> {
    let instrument_count = library.instrument_count(track.drum_type);

    let content = column![
        row![
            text("Selected Track").size(18),
            Space::with_width(Length::Fill),
            text(format!("Track {}", index + 1)).style(secondary_text)
        ]
        .align_y(Alignment::Center),
        row![
            checkbox("Enabled", track.enabled)
                .on_toggle(move |enabled| Message::TrackEnabledChanged(index, enabled)),
            Space::with_width(Length::Fill),
            checkbox("Lock", track.locked)
                .on_toggle(move |locked| Message::TrackLockedChanged(index, locked)),
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
            pick_list(Style::all(), Some(track.style), move |style| {
                Message::TrackStyleChanged(index, style)
            })
            .width(Length::Fill)
            .into()
        ),
        row![
            text("Presence").width(Length::Fixed(72.0)),
            slider(
                GenerationOptions::MIN..=GenerationOptions::MAX,
                track.probability,
                move |probability| Message::TrackProbabilityChanged(index, probability)
            )
            .width(Length::Fill),
            text(format!("{}%", track.probability))
                .width(Length::Fixed(44.0))
                .style(secondary_text),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        row![
            text(format!("{} source patterns", instrument_count))
                .size(12)
                .style(secondary_text),
            Space::with_width(Length::Fill),
            button("Regenerate Track")
                .padding([8, 10])
                .style(subtle_button_style)
                .on_press(Message::RandomizeTrack(index))
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    ]
    .spacing(10);

    container(content)
        .padding(12)
        .width(Length::Fill)
        .style(raised_panel_style)
        .into()
}

fn field<'a>(label: &'static str, control: Element<'a, Message>) -> Element<'a, Message> {
    row![text(label).width(Length::Fixed(64.0)), control]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
}

fn panel_style(_theme: &Theme) -> container::Style {
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

fn track_button_style(selected: bool, status: button::Status) -> button::Style {
    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
    let background = if selected {
        color(0x20, 0x24, 0x2d)
    } else if hovered {
        color(0x18, 0x1b, 0x22)
    } else {
        color(0x14, 0x17, 0x1d)
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: color(0xe8, 0xea, 0xed),
        border: Border {
            color: if selected {
                color(0x6c, 0x8c, 0xff)
            } else {
                color(0x30, 0x36, 0x41)
            },
            width: 1.0,
            radius: 7.0.into(),
        },
        ..button::Style::default()
    }
}

fn subtle_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let background = if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        color(0x2a, 0x30, 0x3a)
    } else {
        color(0x18, 0x1b, 0x22)
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: color(0xe8, 0xea, 0xed),
        border: Border {
            color: color(0x30, 0x36, 0x41),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..button::Style::default()
    }
}

fn secondary_text(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(color(0xa7, 0xaf, 0xbe)),
    }
}

fn success_text(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(color(0x40, 0xd6, 0xb0)),
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}
