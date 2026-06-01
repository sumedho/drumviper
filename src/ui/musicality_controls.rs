use crate::app::Message;
use crate::models::GenerationOptions;
use iced::widget::{row, slider, text};
use iced::{Alignment, Element, Length};

pub fn view(generation_options: GenerationOptions) -> Element<'static, Message> {
    row![
        slider_control(
            format!(
                "Density: {} {}",
                generation_options.density_label(),
                generation_options.density
            ),
            generation_options.density,
            Message::DensityChanged
        ),
        slider_control(
            format!("Complexity: {}", generation_options.complexity),
            generation_options.complexity,
            Message::ComplexityChanged
        ),
        slider_control(
            format!(
                "Fill: {} {}",
                fill_label(generation_options.fill_amount),
                generation_options.fill_amount
            ),
            generation_options.fill_amount,
            Message::FillAmountChanged
        ),
        slider_control(
            format!("Groove: {}", generation_options.groove),
            generation_options.groove,
            Message::GrooveChanged
        ),
        slider_control(
            format!("Humanize: {}", generation_options.humanize),
            generation_options.humanize,
            Message::HumanizeChanged
        )
    ]
    .spacing(18)
    .align_y(Alignment::Center)
    .into()
}

fn slider_control(
    label: String,
    value: u8,
    on_change: fn(u8) -> Message,
) -> Element<'static, Message> {
    row![
        text(label).width(Length::Fixed(148.0)),
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

fn fill_label(fill_amount: u8) -> &'static str {
    match fill_amount {
        0 => "None",
        1..=34 => "Subtle",
        35..=69 => "Medium",
        _ => "Heavy",
    }
}
