use crate::app::Message;
use crate::models::{HitSource, Pattern, TrackConfig, STEPS_PER_BAR, TRACK_COUNT};
use crate::ui::constants::{
    PATTERN_DEFAULT_MARKER_HEIGHT, PATTERN_DEFAULT_MARKER_WIDTH_FACTOR, PATTERN_FILL_MARKER_HEIGHT,
    PATTERN_FILL_MARKER_WIDTH_FACTOR, PATTERN_GHOST_MARKER_HEIGHT,
    PATTERN_GHOST_MARKER_WIDTH_FACTOR, PATTERN_GRID_PADDING, PATTERN_LABEL_GUTTER,
    PATTERN_LABEL_INSET, PATTERN_MAX_ACCENT_MARKER_WIDTH, PATTERN_MAX_MARKER_WIDTH,
    PATTERN_MAX_SMALL_MARKER_WIDTH, PATTERN_MIN_ACCENT_MARKER_WIDTH, PATTERN_MIN_GRID_HEIGHT,
    PATTERN_MIN_MARKER_WIDTH, PATTERN_PANEL_RADIUS, PATTERN_ROW_HEIGHT, PATTERN_TOP_RULER,
};
use iced::alignment;
use iced::border;
use iced::mouse;
use iced::widget::canvas::event;
use iced::widget::canvas::{self, Canvas, Path, Stroke, Text};
use iced::{Color, Element, Length, Pixels, Point, Rectangle, Renderer, Size, Theme};

pub fn view<'a>(tracks: &'a [TrackConfig], pattern: &'a Pattern) -> Element<'a, Message> {
    Canvas::new(PatternGrid { tracks, pattern })
        .width(Length::Fill)
        .height(Length::Fixed(PATTERN_MIN_GRID_HEIGHT))
        .into()
}

#[derive(Debug)]
struct PatternGrid<'a> {
    tracks: &'a [TrackConfig],
    pattern: &'a Pattern,
}

impl<'a> canvas::Program<Message> for PatternGrid<'a> {
    type State = Option<HoverCell>;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas::Event::Mouse(mouse::Event::CursorEntered) => {
                let layout = GridLayout::new(bounds.size(), self.pattern.bars);
                *state = cursor
                    .position_in(bounds)
                    .and_then(|position| layout.hover_cell(position));
                (event::Status::Ignored, None)
            }
            canvas::Event::Mouse(mouse::Event::CursorLeft) => {
                *state = None;
                (event::Status::Ignored, None)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
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
        draw_hover(&mut frame, &layout, state);
        draw_hits(&mut frame, &layout, self.pattern);
        draw_tooltip(&mut frame, &layout, self.pattern, self.tracks, state);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        let layout = GridLayout::new(bounds.size(), self.pattern.bars);
        if cursor
            .position_in(bounds)
            .and_then(|position| layout.hover_cell(position))
            .is_some()
        {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HoverCell {
    track: usize,
    step: u32,
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
        let grid_left = PATTERN_LABEL_GUTTER;
        let grid_top = PATTERN_TOP_RULER;
        let grid_width = (size.width - grid_left - PATTERN_GRID_PADDING).max(steps as f32);
        let grid_height = PATTERN_ROW_HEIGHT * TRACK_COUNT as f32;

        Self {
            grid_left,
            grid_top,
            grid_width,
            grid_height,
            steps,
            cell_width: grid_width / steps.max(1) as f32,
            row_height: PATTERN_ROW_HEIGHT,
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

    fn hover_cell(self, position: Point) -> Option<HoverCell> {
        if position.x < self.grid_left
            || position.x > self.grid_left + self.grid_width
            || position.y < self.grid_top
            || position.y > self.grid_top + self.grid_height
        {
            return None;
        }

        let step = ((position.x - self.grid_left) / self.cell_width).floor() as u32;
        let track = ((position.y - self.grid_top) / self.row_height).floor() as usize;

        (step < self.steps && track < TRACK_COUNT).then_some(HoverCell { track, step })
    }
}

fn draw_background(frame: &mut canvas::Frame, size: Size) {
    frame.fill_rectangle(Point::ORIGIN, size, color(0x11, 0x13, 0x18, 1.0));

    let panel = Path::rounded_rectangle(
        Point::new(PATTERN_GRID_PADDING / 2.0, PATTERN_GRID_PADDING / 2.0),
        Size::new(
            size.width - PATTERN_GRID_PADDING,
            size.height - PATTERN_GRID_PADDING,
        ),
        border::Radius::from(PATTERN_PANEL_RADIUS),
    );
    frame.fill(&panel, color(0x18, 0x1b, 0x22, 1.0));
}

fn draw_track_rows(frame: &mut canvas::Frame, layout: &GridLayout, tracks: &[TrackConfig]) {
    for track_index in 0..TRACK_COUNT {
        let y = layout.y_for_track(track_index);
        let row_color = if track_index % 2 == 0 {
            color(0x1d, 0x22, 0x2b, 1.0)
        } else {
            color(0x19, 0x1e, 0x26, 1.0)
        };

        frame.fill_rectangle(
            Point::new(PATTERN_GRID_PADDING, y),
            Size::new(
                PATTERN_LABEL_GUTTER - PATTERN_GRID_PADDING,
                layout.row_height,
            ),
            color(0x20, 0x24, 0x2d, 1.0),
        );

        frame.fill_rectangle(
            Point::new(layout.grid_left, y),
            Size::new(layout.grid_width, layout.row_height),
            row_color,
        );

        let label = tracks
            .get(track_index)
            .map(|track| track.drum_type.to_string())
            .unwrap_or_else(|| format!("Track {}", track_index + 1));
        frame.fill_text(Text {
            content: label,
            position: Point::new(
                PATTERN_GRID_PADDING + PATTERN_LABEL_INSET,
                y + layout.row_height / 2.0,
            ),
            color: color(0xe8, 0xea, 0xed, 0.94),
            size: Pixels(13.0),
            vertical_alignment: alignment::Vertical::Center,
            ..Text::default()
        });

        let line = Path::line(
            Point::new(PATTERN_GRID_PADDING, y + layout.row_height),
            Point::new(layout.grid_left + layout.grid_width, y + layout.row_height),
        );
        frame.stroke(
            &line,
            Stroke::default().with_color(color(0x30, 0x36, 0x41, 0.86)),
        );
    }

    let gutter_line = Path::line(
        Point::new(layout.grid_left, layout.grid_top),
        Point::new(layout.grid_left, layout.grid_top + layout.grid_height),
    );
    frame.stroke(
        &gutter_line,
        Stroke::default()
            .with_width(1.5)
            .with_color(color(0x6c, 0x8c, 0xff, 0.48)),
    );
}

fn draw_timeline(frame: &mut canvas::Frame, layout: &GridLayout) {
    for step in 0..=layout.steps {
        let x = layout.x_for_step(step);
        let is_bar = step % STEPS_PER_BAR == 0;
        let is_beat = step % 4 == 0;
        let (width, alpha) = if is_bar {
            (2.3, 0.95)
        } else if is_beat {
            (1.0, 0.5)
        } else {
            (1.0, 0.18)
        };

        let line = Path::line(
            Point::new(x, layout.grid_top - 10.0),
            Point::new(x, layout.grid_top + layout.grid_height),
        );
        frame.stroke(
            &line,
            Stroke::default()
                .with_width(width)
                .with_color(color(0xa7, 0xaf, 0xbe, alpha)),
        );

        if is_bar && step < layout.steps {
            let bar = step / STEPS_PER_BAR + 1;
            frame.fill_text(Text {
                content: format!("Bar {bar}"),
                position: Point::new(x + 5.0, 17.0),
                color: color(0xe8, 0xea, 0xed, 0.92),
                size: Pixels(11.0),
                ..Text::default()
            });
        } else if is_beat && step < layout.steps {
            let beat = step / 4 % 4 + 1;
            frame.fill_text(Text {
                content: beat.to_string(),
                position: Point::new(x + 3.0, 31.0),
                color: color(0xa7, 0xaf, 0xbe, 0.72),
                size: Pixels(10.0),
                ..Text::default()
            });
        }
    }
}

fn draw_hover(frame: &mut canvas::Frame, layout: &GridLayout, hover: &Option<HoverCell>) {
    let Some(hover) = hover else {
        return;
    };

    let row_y = layout.y_for_track(hover.track);
    frame.fill_rectangle(
        Point::new(layout.grid_left, row_y),
        Size::new(layout.grid_width, layout.row_height),
        color(0x6c, 0x8c, 0xff, 0.09),
    );

    frame.fill_rectangle(
        Point::new(layout.x_for_step(hover.step), layout.grid_top),
        Size::new(layout.cell_width.max(2.0), layout.grid_height),
        color(0x40, 0xd6, 0xb0, 0.12),
    );
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
            HitSource::Ghost => (layout.cell_width * PATTERN_GHOST_MARKER_WIDTH_FACTOR)
                .clamp(PATTERN_MIN_MARKER_WIDTH, PATTERN_MAX_SMALL_MARKER_WIDTH),
            HitSource::Fill => (layout.cell_width * PATTERN_FILL_MARKER_WIDTH_FACTOR).clamp(
                PATTERN_MIN_ACCENT_MARKER_WIDTH,
                PATTERN_MAX_ACCENT_MARKER_WIDTH,
            ),
            _ => (layout.cell_width * PATTERN_DEFAULT_MARKER_WIDTH_FACTOR)
                .clamp(PATTERN_MIN_MARKER_WIDTH, PATTERN_MAX_MARKER_WIDTH),
        };
        let marker_height = match hit.source {
            HitSource::Ghost => PATTERN_GHOST_MARKER_HEIGHT,
            HitSource::Fill => PATTERN_FILL_MARKER_HEIGHT,
            _ if hit.velocity >= 104 => PATTERN_DEFAULT_MARKER_HEIGHT + 3.0,
            _ => PATTERN_DEFAULT_MARKER_HEIGHT,
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

fn draw_tooltip(
    frame: &mut canvas::Frame,
    layout: &GridLayout,
    pattern: &Pattern,
    tracks: &[TrackConfig],
    hover: &Option<HoverCell>,
) {
    let Some(hover) = hover else {
        return;
    };

    let track_name = tracks
        .get(hover.track)
        .map(|track| track.drum_type.to_string())
        .unwrap_or_else(|| format!("Track {}", hover.track + 1));
    let bar = hover.step / STEPS_PER_BAR + 1;
    let beat = hover.step % STEPS_PER_BAR / 4 + 1;
    let subdivision = hover.step % 4 + 1;
    let hit = hit_at_step(pattern, hover.track, hover.step);
    let content = match hit {
        Some(hit) => format!(
            "Bar {bar} Beat {beat}.{subdivision} - {track_name} velocity {} ({})",
            hit.velocity,
            source_label(hit.source)
        ),
        None => format!("Bar {bar} Beat {beat}.{subdivision} - {track_name}"),
    };

    let tooltip_width = (content.len() as f32 * 7.0 + 20.0).clamp(220.0, 420.0);
    let x = (layout.x_for_step(hover.step) + 10.0)
        .min(layout.grid_left + layout.grid_width - tooltip_width - 8.0);
    let y = (layout.y_for_track(hover.track) - 34.0).max(8.0);
    let tooltip = Path::rounded_rectangle(
        Point::new(x, y),
        Size::new(tooltip_width, 28.0),
        border::Radius::from(6.0),
    );
    frame.fill(&tooltip, color(0x20, 0x24, 0x2d, 0.97));
    frame.stroke(
        &tooltip,
        Stroke::default().with_color(color(0x30, 0x36, 0x41, 1.0)),
    );
    frame.fill_text(Text {
        content,
        position: Point::new(x + 10.0, y + 14.0),
        color: color(0xe8, 0xea, 0xed, 0.96),
        size: Pixels(12.0),
        vertical_alignment: alignment::Vertical::Center,
        ..Text::default()
    });
}

fn hit_at_step(pattern: &Pattern, track: usize, step: u32) -> Option<&crate::models::Hit> {
    let ticks_per_step = u32::from(pattern.ppq) / 4;
    pattern
        .hits
        .iter()
        .find(|hit| hit.track == track && hit.tick / ticks_per_step == step)
}

fn source_label(source: HitSource) -> &'static str {
    match source {
        HitSource::SourcePattern => "source",
        HitSource::StyleRule => "generated",
        HitSource::Ghost => "ghost",
        HitSource::Fill => "fill",
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
    use crate::models::{PPQ, TICKS_PER_STEP};

    #[test]
    fn layout_fits_all_steps_for_sixteen_bars() {
        let layout = GridLayout::new(Size::new(1_200.0, 400.0), 16);

        assert_eq!(layout.steps, 256);
        assert!(layout.cell_width > 0.0);
        assert!(layout.x_for_step(256) <= 1_200.0 - PATTERN_GRID_PADDING + 0.01);
    }

    #[test]
    fn tick_position_maps_to_sixteenth_steps() {
        let layout = GridLayout::new(Size::new(1_000.0, 400.0), 8);
        let pattern = Pattern::empty(8, PPQ);
        let step_four_x = layout.x_for_step(4);

        assert!((layout.x_for_tick(&pattern, TICKS_PER_STEP * 4) - step_four_x).abs() < 0.01);
    }
}
