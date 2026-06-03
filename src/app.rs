use crate::drag_export::{self, DragExportResult};
use crate::generator;
use crate::midi;
use crate::models::{
    BarLength, DrumType, GenerationOptions, Pattern, SongSection, Style, TrackConfig,
    DEFAULT_TEMPO_BPM, TRACK_COUNT,
};
use crate::patterns::SourcePatternLibrary;
use crate::source_patterns::SourceSection;
use crate::ui;
use iced::{Element, Subscription, Task, Theme};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    TrackEnabledChanged(usize, bool),
    TrackLockedChanged(usize, bool),
    TrackProbabilityChanged(usize, u8),
    TrackDrumChanged(usize, DrumType),
    TrackStyleChanged(usize, Style),
    TrackSelected(usize),
    RandomizeTrack(usize),
    RandomizeAll,
    LengthChanged(BarLength),
    TempoInputChanged(String),
    DensityChanged(u8),
    ComplexityChanged(u8),
    FillAmountChanged(u8),
    VariationChanged(u8),
    GrooveChanged(u8),
    HumanizeChanged(u8),
    GlobalStyleChanged(Style),
    GlobalSectionChanged(SongSection),
    ExportMidi,
    DragMidi,
    DragMidiFinished(DragExportResult),
    WindowOpened(iced::window::Id),
}

pub struct DrumViper {
    library: SourcePatternLibrary,
    tracks: Vec<TrackConfig>,
    pattern: Pattern,
    length: BarLength,
    tempo: u16,
    tempo_input: String,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
    selected_track_index: usize,
    window_id: Option<iced::window::Id>,
    last_drag_midi_path: Option<PathBuf>,
    status: String,
}

impl DrumViper {
    pub fn new() -> (Self, Task<Message>) {
        let (library, status) = match SourcePatternLibrary::load_embedded() {
            Ok(library) => (
                library,
                String::from("Loaded embedded Pocket Operations patterns."),
            ),
            Err(error) => (
                SourcePatternLibrary::empty(),
                format!("Pattern library failed to load: {error}"),
            ),
        };

        let tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        let length = BarLength::Four;
        let generation_options = GenerationOptions::default();
        let pattern = generator::generate_pattern(&library, &tracks, length, &generation_options);

        (
            Self {
                library,
                tracks,
                pattern,
                length,
                tempo: DEFAULT_TEMPO_BPM,
                tempo_input: DEFAULT_TEMPO_BPM.to_string(),
                global_style: Style::Source(SourceSection::BasicPatterns),
                global_section: SongSection::Verse,
                generation_options,
                selected_track_index: 0,
                window_id: None,
                last_drag_midi_path: None,
                status,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TrackEnabledChanged(index, enabled) => {
                if let Some(track) = self.tracks.get_mut(index) {
                    track.enabled = enabled;
                    self.regenerate_all();
                }
            }
            Message::TrackLockedChanged(index, locked) => {
                if let Some(track) = self.tracks.get_mut(index) {
                    track.locked = locked;
                    self.status = if locked {
                        format!("Locked track {}.", index + 1)
                    } else {
                        format!("Unlocked track {}.", index + 1)
                    };
                }
            }
            Message::TrackProbabilityChanged(index, probability) => {
                if let Some(track) = self.tracks.get_mut(index) {
                    track.probability = probability;
                    if track.locked {
                        self.status = format!(
                            "Changed track {} amount. Unlock or randomize the track to regenerate it.",
                            index + 1
                        );
                    } else {
                        self.regenerate_track(index);
                    }
                }
            }
            Message::TrackDrumChanged(index, drum_type) => {
                if let Some(track) = self.tracks.get_mut(index) {
                    track.drum_type = drum_type;
                    if track.locked {
                        self.status = format!(
                            "Changed track {} drum. Unlock or randomize the track to regenerate it.",
                            index + 1
                        );
                    } else {
                        self.regenerate_track(index);
                    }
                }
            }
            Message::TrackStyleChanged(index, style) => {
                if let Some(track) = self.tracks.get_mut(index) {
                    track.style = style;
                    if track.locked {
                        self.status = format!(
                            "Changed track {} style. Unlock or randomize the track to regenerate it.",
                            index + 1
                        );
                    } else {
                        self.regenerate_track(index);
                    }
                }
            }
            Message::TrackSelected(index) => {
                if index < self.tracks.len() {
                    self.selected_track_index = index;
                }
            }
            Message::RandomizeTrack(index) => {
                self.regenerate_track(index);
            }
            Message::RandomizeAll => {
                self.regenerate_all();
            }
            Message::LengthChanged(length) => {
                self.length = length;
                self.regenerate_all();
            }
            Message::TempoInputChanged(value) => {
                self.update_tempo_input(value);
            }
            Message::DensityChanged(density) => {
                self.generation_options.density = density;
                self.regenerate_all();
            }
            Message::ComplexityChanged(complexity) => {
                self.generation_options.complexity = complexity;
                self.regenerate_all();
            }
            Message::FillAmountChanged(fill_amount) => {
                self.generation_options.fill_amount = fill_amount;
                self.regenerate_all();
            }
            Message::VariationChanged(variation) => {
                self.generation_options.variation = variation;
                self.regenerate_all();
            }
            Message::GrooveChanged(groove) => {
                self.generation_options.groove = groove;
                self.regenerate_all();
            }
            Message::HumanizeChanged(humanize) => {
                self.generation_options.humanize = humanize;
                self.regenerate_all();
            }
            Message::GlobalStyleChanged(style) => {
                self.global_style = style;
                self.regenerate_all();
            }
            Message::GlobalSectionChanged(section) => {
                self.global_section = section;
                self.regenerate_all();
            }
            Message::ExportMidi => {
                self.export_midi();
            }
            Message::DragMidi => {
                return self.drag_midi();
            }
            Message::DragMidiFinished(result) => {
                self.handle_drag_midi_result(result);
            }
            Message::WindowOpened(id) => {
                self.window_id = Some(id);
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::root(
            &self.tracks,
            &self.pattern,
            self.length,
            self.tempo,
            &self.tempo_input,
            self.global_style,
            self.global_section,
            self.generation_options,
            self.selected_track_index,
            &self.status,
            &self.library,
        )
    }

    pub fn theme(&self) -> Theme {
        ui::theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::window::open_events().map(Message::WindowOpened)
    }

    fn regenerate_all(&mut self) {
        let tracks = self.globalized_tracks();
        self.pattern = generator::generate_pattern_preserving_locked(
            &self.library,
            &self.pattern,
            &tracks,
            self.length,
            &self.generation_options,
        );
        self.status = format!(
            "Generated {} bars using {} / {}, {} density, fill {}, variation {}, track amounts.",
            self.length.bars(),
            self.global_style,
            self.global_section,
            self.generation_options.density_label(),
            self.generation_options.fill_amount,
            self.generation_options.variation
        );
    }

    fn regenerate_track(&mut self, index: usize) {
        if let Some(track) = self.tracks.get(index) {
            let mut track = track.clone();
            track.section = self.global_section;
            self.pattern = generator::generate_single_track(
                &self.library,
                &self.pattern,
                index,
                &track,
                self.length,
                &self.generation_options,
            );
            self.status = format!(
                "Regenerated track {} using {} / {}.",
                index + 1,
                track.style,
                self.global_section
            );
        }
    }

    fn globalized_tracks(&self) -> Vec<TrackConfig> {
        tracks_with_global_settings(&self.tracks, self.global_style, self.global_section)
    }

    fn export_midi(&mut self) {
        let Some(path) = FileDialog::new()
            .set_title("Export DrumViper MIDI")
            .add_filter("MIDI file", &["mid", "midi"])
            .set_file_name("drumviper-pattern.mid")
            .save_file()
        else {
            self.status = String::from("Export cancelled.");
            return;
        };

        match midi::export_midi(&self.pattern, self.tempo, &path) {
            Ok(()) => {
                self.status = format!("Exported MIDI to {}.", path.display());
            }
            Err(error) => {
                self.status = format!("MIDI export failed: {error}");
            }
        }
    }

    fn update_tempo_input(&mut self, value: String) {
        if value.is_empty() {
            self.tempo_input.clear();
            return;
        }

        if !value.chars().all(|character| character.is_ascii_digit()) {
            return;
        }

        self.tempo_input = value;

        if let Ok(tempo) = self.tempo_input.parse::<u16>() {
            if (crate::models::MIN_TEMPO_BPM..=crate::models::MAX_TEMPO_BPM).contains(&tempo) {
                self.tempo = tempo;
            }
        }
    }

    fn drag_midi(&mut self) -> Task<Message> {
        let path = match drag_export::write_drag_midi(&self.pattern, self.tempo) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("MIDI drag export failed: {error}");
                return Task::none();
            }
        };

        self.last_drag_midi_path = Some(path.clone());

        let Some(window_id) = self.window_id else {
            self.status = format!(
                "Prepared MIDI for drag at {}, but the app window is not ready.",
                path.display()
            );
            return Task::none();
        };

        self.status = format!("Prepared MIDI drag file: {}.", path.display());

        iced::window::run_with_handle(window_id, move |handle| {
            match drag_export::begin_native_file_drag(handle, &path) {
                Ok(()) => DragExportResult::Started(path),
                Err(_) if !cfg!(target_os = "macos") => DragExportResult::Unavailable(path),
                Err(error) => DragExportResult::Failed(error),
            }
        })
        .map(Message::DragMidiFinished)
    }

    fn handle_drag_midi_result(&mut self, result: DragExportResult) {
        match result {
            DragExportResult::Started(path) => {
                self.status = format!("Started MIDI drag from {}.", path.display());
            }
            DragExportResult::Unavailable(path) => {
                self.status = format!(
                    "Prepared MIDI at {}. Native drag is unavailable on this platform.",
                    path.display()
                );
            }
            DragExportResult::Failed(error) => {
                self.status = format!("MIDI drag failed: {error}");
            }
        }
    }
}

fn tracks_with_global_settings(
    tracks: &[TrackConfig],
    style: Style,
    section: SongSection,
) -> Vec<TrackConfig> {
    tracks
        .iter()
        .cloned()
        .map(|mut track| {
            track.style = style;
            track.section = section;
            track
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_overrides_preserve_track_drums_and_enabled_state() {
        let mut tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        tracks[1].enabled = false;
        tracks[1].locked = true;
        tracks[1].probability = 35;
        tracks[2].style = Style::Trap;
        tracks[2].section = SongSection::Intro;

        let overridden =
            tracks_with_global_settings(&tracks, Style::Techno, SongSection::HighEnergy);

        assert_eq!(overridden[1].enabled, tracks[1].enabled);
        assert_eq!(overridden[1].locked, tracks[1].locked);
        assert_eq!(overridden[1].probability, tracks[1].probability);
        assert_eq!(overridden[2].drum_type, tracks[2].drum_type);
        assert!(overridden.iter().all(|track| track.style == Style::Techno));
        assert!(overridden
            .iter()
            .all(|track| track.section == SongSection::HighEnergy));
    }

    #[test]
    fn selected_track_defaults_to_first_track_and_ignores_invalid_indexes() {
        let (mut app, _) = DrumViper::new();

        assert_eq!(app.selected_track_index, 0);

        let _ = app.update(Message::TrackSelected(3));

        assert_eq!(app.selected_track_index, 3);

        let _ = app.update(Message::TrackSelected(TRACK_COUNT + 5));

        assert_eq!(app.selected_track_index, 3);
    }

    #[test]
    fn app_defaults_to_four_bar_patterns() {
        let (app, _) = DrumViper::new();

        assert_eq!(app.length, BarLength::Four);
        assert_eq!(app.pattern.bars, 4);
    }

    #[test]
    fn tempo_input_accepts_only_integer_bpm_values() {
        let (mut app, _) = DrumViper::new();

        let _ = app.update(Message::TempoInputChanged(String::from("140")));
        assert_eq!(app.tempo, 140);
        assert_eq!(app.tempo_input, "140");

        let _ = app.update(Message::TempoInputChanged(String::from("1404")));
        assert_eq!(app.tempo, 140);
        assert_eq!(app.tempo_input, "1404");

        let _ = app.update(Message::TempoInputChanged(String::from("14.5")));
        assert_eq!(app.tempo, 140);
        assert_eq!(app.tempo_input, "1404");

        let _ = app.update(Message::TempoInputChanged(String::from("abc")));
        assert_eq!(app.tempo, 140);
        assert_eq!(app.tempo_input, "1404");

        let _ = app.update(Message::TempoInputChanged(String::from("120")));
        assert_eq!(app.tempo, 120);
        assert_eq!(app.tempo_input, "120");
    }
}
