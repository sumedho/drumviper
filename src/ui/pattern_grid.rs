use crate::app::Message;
use crate::models::{HitSource, Pattern, TrackConfig, STEPS_PER_BAR, TRACK_COUNT};
use iced::alignment;
use iced::border;
use iced::mouse;
use iced::widget::canvas::{self, Canvas, Path, Stroke, Text};
use iced::{Color, Element, Length, Pixels, Point, Rectangle, Renderer, Size, Theme};

const LABEL_GUTTER: f32 = 132.0;
const TOP_RULER: f32 = 38.0;
const ROW_HEIGHT: f32 = 38.0;
const GRID_PADDING: f32 = 12.0;
const MIN_GRID_HEIGHT: f32 = TOP_RULER + ROW_HEIGHT * TRACK_COUNT as f32 + GRID_PADDING * 2.0;

pub fn view<'a>(tracks: &'a [TrackConfig], pattern: &'a Pattern) -> Element<'a, Message> {
    Canvas::new(PatternGrid { tracks, pattern })
        .width(Length::Fill)
        .height(Length::Fixed(MIN_GRID_HEIGHT))
        .into()
}

#[derive(Debug)]
struct PatternGrid<'a> {
    tracks: &'a [TrackConfig],
    pattern: &'a Pattern,
}

impl<'a> canvas::Program<Message> for PatternGrid<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let layout = GridLayout::new(bounds.size(), self.pattern.bars);

        draw_background(&mut frame, bounds.size());
        draw_track_rows(&mut frame, &layout, self.tracks);
        draw_timeline(&mut frame, &layout);
        draw_hits(&mut frame, &layout, self.pattern);

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone, Copy)]
struct GridLayout {
    grid_left: f32,
    grid_top: f32,
    grid_width: f32,
    grid_height: f32,
    steps: u32,
    cell_width: f32,
    row_height: f32,
}

impl GridLayout {
    fn new(size: Size, bars: u32) -> Self {
        let steps = bars * STEPS_PER_BAR;
        let grid_left = LABEL_GUTTER;
        let grid_top = TOP_RULER;
        let grid_width = (size.width - grid_left - GRID_PADDING).max(steps as f32);
        let grid_height = ROW_HEIGHT * TRACK_COUNT as f32;

        Self {
            grid_left,
            grid_top,
            grid_width,
            grid_height,
            steps,
            cell_width: grid_width / steps.max(1) as f32,
            row_height: ROW_HEIGHT,
        }
    }

    fn x_for_step(self, step: u32) -> f32 {
        self.grid_left + step as f32 * self.cell_width
    }

    fn x_for_tick(self, pattern: &Pattern, tick: u32) -> f32 {
        let ticks_per_step = f32::from(pattern.ppq) / 4.0;
        self.grid_left + tick as f32 / ticks_per_step * self.cell_width
    }

    fn y_for_track(self, track: usize) -> f32 {
        self.grid_top + track as f32 * self.row_height
    }
}

fn draw_background(frame: &mut canvas::Frame, size: Size) {
    frame.fill_rectangle(Point::ORIGIN, size, color(0x12, 0x16, 0x1f, 1.0));

    let panel = Path::rounded_rectangle(
        Point::new(GRID_PADDING / 2.0, GRID_PADDING / 2.0),
        Size::new(size.width - GRID_PADDING, size.height - GRID_PADDING),
        border::Radius::from(7.0),
    );
    frame.fill(&panel, color(0x18, 0x1e, 0x29, 1.0));
}

fn draw_track_rows(frame: &mut canvas::Frame, layout: &GridLayout, tracks: &[TrackConfig]) {
    for track_index in 0..TRACK_COUNT {
        let y = layout.y_for_track(track_index);
        let row_color = if track_index % 2 == 0 {
            color(0x1d, 0x24, 0x30, 1.0)
        } else {
            color(0x19, 0x20, 0x2b, 1.0)
        };

        frame.fill_rectangle(
            Point::new(GRID_PADDING, y),
            Size::new(
                layout.grid_width + LABEL_GUTTER - GRID_PADDING,
                layout.row_height,
            ),
            row_color,
        );

        let label = tracks
            .get(track_index)
            .map(|track| track.drum_type.to_string())
            .unwrap_or_else(|| format!("Track {}", track_index + 1));
        frame.fill_text(Text {
            content: label,
            position: Point::new(GRID_PADDING + 12.0, y + layout.row_height / 2.0),
            color: color(0xd6, 0xde, 0xea, 0.92),
            size: Pixels(13.0),
            vertical_alignment: alignment::Vertical::Center,
            ..Text::default()
        });

        let line = Path::line(
            Point::new(GRID_PADDING, y + layout.row_height),
            Point::new(layout.grid_left + layout.grid_width, y + layout.row_height),
        );
        frame.stroke(
            &line,
            Stroke::default().with_color(color(0x2a, 0x34, 0x44, 0.75)),
        );
    }
}

fn draw_timeline(frame: &mut canvas::Frame, layout: &GridLayout) {
    for step in 0..=layout.steps {
        let x = layout.x_for_step(step);
        let is_bar = step % STEPS_PER_BAR == 0;
        let is_beat = step % 4 == 0;
        let (width, alpha) = if is_bar {
            (2.0, 0.95)
        } else if is_beat {
            (1.0, 0.48)
        } else {
            (1.0, 0.15)
        };

        let line = Path::line(
            Point::new(x, layout.grid_top - 10.0),
            Point::new(x, layout.grid_top + layout.grid_height),
        );
        frame.stroke(
            &line,
            Stroke::default()
                .with_width(width)
                .with_color(color(0x79, 0x87, 0x9a, alpha)),
        );

        if is_bar && step < layout.steps {
            let bar = step / STEPS_PER_BAR + 1;
            frame.fill_text(Text {
                content: format!("Bar {bar}"),
                position: Point::new(x + 5.0, 17.0),
                color: color(0xc3, 0xcc, 0xd8, 0.92),
                size: Pixels(11.0),
                ..Text::default()
            });
        } else if is_beat && step < layout.steps {
            let beat = step / 4 % 4 + 1;
            frame.fill_text(Text {
                content: beat.to_string(),
                position: Point::new(x + 3.0, 31.0),
                color: color(0x8f, 0x9b, 0xaa, 0.7),
                size: Pixels(10.0),
                ..Text::default()
            });
        }
    }
}

fn draw_hits(frame: &mut canvas::Frame, layout: &GridLayout, pattern: &Pattern) {
    for hit in &pattern.hits {
        if hit.track >= TRACK_COUNT {
            continue;
        }

        let x = layout.x_for_tick(pattern, hit.tick);
        let y = layout.y_for_track(hit.track);
        let intensity = (f32::from(hit.velocity) / 127.0).clamp(0.25, 1.0);
        let marker_width = match hit.source {
            HitSource::Ghost => (layout.cell_width * 0.42).clamp(2.0, 8.0),
            HitSource::Fill => (layout.cell_width * 0.74).clamp(2.5, 12.0),
            _ => (layout.cell_width * 0.68).clamp(2.0, 11.0),
        };
        let marker_height = match hit.source {
            HitSource::Ghost => 8.0,
            HitSource::Fill => 18.0,
            _ => 15.0,
        };

        let marker = Path::rounded_rectangle(
            Point::new(
                x + (layout.cell_width - marker_width) / 2.0,
                y + (layout.row_height - marker_height) / 2.0,
            ),
            Size::new(marker_width, marker_height),
            border::Radius::from(3.0),
        );
        frame.fill(&marker, hit_color(hit.source, intensity));
    }
}

fn hit_color(source: HitSource, intensity: f32) -> Color {
    match source {
        HitSource::SourcePattern => color(0x43, 0xd9, 0xb8, 0.46 + intensity * 0.5),
        HitSource::StyleRule => color(0x7a, 0xa2, 0xff, 0.42 + intensity * 0.5),
        HitSource::Ghost => color(0x9b, 0xa7, 0xb8, 0.22 + intensity * 0.32),
        HitSource::Fill => color(0xff, 0xc8, 0x57, 0.5 + intensity * 0.48),
    }
}

fn color(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
    Color::from_rgba8(red, green, blue, alpha.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_fits_all_steps_for_sixteen_bars() {
        let layout = GridLayout::new(Size::new(1_200.0, 400.0), 16);

        assert_eq!(layout.steps, 256);
        assert!(layout.cell_width > 0.0);
        assert!(layout.x_for_step(256) <= 1_200.0 - GRID_PADDING + 0.01);
    }

    #[test]
    fn tick_position_maps_to_sixteenth_steps() {
        let layout = GridLayout::new(Size::new(1_000.0, 400.0), 8);
        let pattern = Pattern::empty(8, 480);
        let step_four_x = layout.x_for_step(4);

        assert!((layout.x_for_tick(&pattern, 480) - step_four_x).abs() < 0.01);
    }
}
