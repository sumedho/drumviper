use crate::generator;
use crate::midi;
use crate::models::{
    BarLength, DrumType, GenerationOptions, Pattern, SongSection, Style, TrackConfig,
    DEFAULT_TEMPO_BPM, TRACK_COUNT,
};
use crate::patterns::SourcePatternLibrary;
use crate::ui;
use iced::{Element, Task, Theme};
use rfd::FileDialog;

#[derive(Debug, Clone)]
pub enum Message {
    TrackEnabledChanged(usize, bool),
    TrackLockedChanged(usize, bool),
    TrackProbabilityChanged(usize, u8),
    TrackDrumChanged(usize, DrumType),
    TrackStyleChanged(usize, Style),
    RandomizeTrack(usize),
    RandomizeAll,
    LengthChanged(BarLength),
    TempoChanged(u16),
    DensityChanged(u8),
    ComplexityChanged(u8),
    FillAmountChanged(u8),
    GrooveChanged(u8),
    HumanizeChanged(u8),
    GlobalStyleChanged(Style),
    GlobalSectionChanged(SongSection),
    ExportMidi,
}

pub struct DrumViper {
    library: SourcePatternLibrary,
    tracks: Vec<TrackConfig>,
    pattern: Pattern,
    length: BarLength,
    tempo: u16,
    global_style: Style,
    global_section: SongSection,
    generation_options: GenerationOptions,
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
        let length = BarLength::Eight;
        let generation_options = GenerationOptions::default();
        let pattern = generator::generate_pattern(&library, &tracks, length, &generation_options);

        (
            Self {
                library,
                tracks,
                pattern,
                length,
                tempo: DEFAULT_TEMPO_BPM,
                global_style: Style::SourceLibrary,
                global_section: SongSection::Verse,
                generation_options,
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
            Message::TempoChanged(tempo) => {
                self.tempo = tempo;
            }
            Message::DensityChanged(density) => {
                self.generation_options.density = density;
            }
            Message::ComplexityChanged(complexity) => {
                self.generation_options.complexity = complexity;
            }
            Message::FillAmountChanged(fill_amount) => {
                self.generation_options.fill_amount = fill_amount;
            }
            Message::GrooveChanged(groove) => {
                self.generation_options.groove = groove;
            }
            Message::HumanizeChanged(humanize) => {
                self.generation_options.humanize = humanize;
            }
            Message::GlobalStyleChanged(style) => {
                self.global_style = style;
            }
            Message::GlobalSectionChanged(section) => {
                self.global_section = section;
            }
            Message::ExportMidi => {
                self.export_midi();
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
            self.global_style,
            self.global_section,
            self.generation_options,
            &self.status,
            &self.library,
        )
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
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
            "Generated {} bars using {} / {}, {} density, fill {}, track amounts.",
            self.length.bars(),
            self.global_style,
            self.global_section,
            self.generation_options.density_label(),
            self.generation_options.fill_amount
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
                "Randomized track {} using {} / {}.",
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
}
