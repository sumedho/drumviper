use std::fmt;

pub const PPQ: u16 = 480;
pub const BEATS_PER_BAR: u32 = 4;
pub const STEPS_PER_BEAT: u32 = 4;
pub const STEPS_PER_BAR: u32 = BEATS_PER_BAR * STEPS_PER_BEAT;
pub const TICKS_PER_STEP: u32 = PPQ as u32 / STEPS_PER_BEAT;
pub const DEFAULT_NOTE_DURATION_TICKS: u32 = TICKS_PER_STEP / 2;
pub const FILL_NOTE_DURATION_TICKS: u32 = DEFAULT_NOTE_DURATION_TICKS / 2;
pub const FINAL_BEAT_STEPS: [u8; 4] = [13, 14, 15, 16];
pub const TRACK_COUNT: usize = 8;

pub const GHOST_VELOCITY_OFFSET: i16 = -42;
pub const FILL_VELOCITY_OFFSET: i16 = -8;
pub const FILL_SUBDIVISION_VELOCITY_REDUCTION: u8 = 18;
pub const ACCENT_LANE_VELOCITY_BOOST: u8 = 18;
pub const ACCENT_STEP_VELOCITY_BOOST: i16 = 10;
pub const PHRASE_START_VELOCITY_BOOST: i16 = 8;
pub const PHRASE_RESPONSE_VELOCITY_BOOST: i16 = 5;
pub const TURNAROUND_VELOCITY_BOOST: i16 = 12;
pub const MAX_HUMANIZE_TIMING_OFFSET_TICKS: f32 = 18.0;
pub const MAX_HUMANIZE_VELOCITY_OFFSET: f32 = 12.0;
pub const DEFAULT_TEMPO_BPM: u16 = 120;
pub const MIN_TEMPO_BPM: u16 = 60;
pub const MAX_TEMPO_BPM: u16 = 180;
pub const MICROTIMING_SMALL_OFFSET_TICKS: i32 = 6;
pub const MICROTIMING_MEDIUM_OFFSET_TICKS: i32 = 12;
pub const MICROTIMING_LARGE_OFFSET_TICKS: i32 = 24;
pub const MICROTIMING_SHUFFLE_OFFSET_TICKS: i32 = 18;
pub const MICROTIMING_BROKEN_OFFSET_TICKS: i32 = 14;
pub const MICROTIMING_ONE_DROP_OFFSET_TICKS: i32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarLength {
    Four,
    Eight,
    Sixteen,
}

impl BarLength {
    pub const ALL: [Self; 3] = [Self::Four, Self::Eight, Self::Sixteen];

    pub fn bars(self) -> u32 {
        match self {
            Self::Four => 4,
            Self::Eight => 8,
            Self::Sixteen => 16,
        }
    }
}

impl fmt::Display for BarLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bars", self.bars())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrumType {
    BassDrum,
    Snare,
    LowTom,
    Rimshot,
    MediumTom,
    Cowbell,
    HighTom,
    Cymbal,
    HandClap,
    OpenHat,
    Shaker,
    ClosedHat,
    Accent,
}

impl DrumType {
    pub const ALL: [Self; 13] = [
        Self::BassDrum,
        Self::Snare,
        Self::LowTom,
        Self::Rimshot,
        Self::MediumTom,
        Self::Cowbell,
        Self::HighTom,
        Self::Cymbal,
        Self::HandClap,
        Self::OpenHat,
        Self::Shaker,
        Self::ClosedHat,
        Self::Accent,
    ];

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "BD" => Some(Self::BassDrum),
            "SN" => Some(Self::Snare),
            "LT" => Some(Self::LowTom),
            "RS" => Some(Self::Rimshot),
            "MT" => Some(Self::MediumTom),
            "CB" => Some(Self::Cowbell),
            "HT" => Some(Self::HighTom),
            "CY" => Some(Self::Cymbal),
            "CL" | "HC" => Some(Self::HandClap),
            "OH" => Some(Self::OpenHat),
            "SH" => Some(Self::Shaker),
            "CH" => Some(Self::ClosedHat),
            "AC" => Some(Self::Accent),
            _ => None,
        }
    }
}

impl fmt::Display for DrumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::BassDrum => "Bass Drum",
            Self::Snare => "Snare",
            Self::LowTom => "Low Tom",
            Self::Rimshot => "Rimshot",
            Self::MediumTom => "Medium Tom",
            Self::Cowbell => "Cowbell",
            Self::HighTom => "High Tom",
            Self::Cymbal => "Cymbal",
            Self::HandClap => "Hand Clap",
            Self::OpenHat => "Open Hi-Hat",
            Self::Shaker => "Shaker",
            Self::ClosedHat => "Closed Hi-Hat",
            Self::Accent => "Accent",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Style {
    SourceLibrary,
    HipHop,
    Trap,
    Drill,
    House,
    Techno,
    Electro,
    UkGarage,
    Breakbeat,
    JungleDnb,
}

impl Style {
    pub const ALL: [Self; 10] = [
        Self::SourceLibrary,
        Self::HipHop,
        Self::Trap,
        Self::Drill,
        Self::House,
        Self::Techno,
        Self::Electro,
        Self::UkGarage,
        Self::Breakbeat,
        Self::JungleDnb,
    ];
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::SourceLibrary => "Source Library",
            Self::HipHop => "Hip-Hop",
            Self::Trap => "Trap",
            Self::Drill => "Drill",
            Self::House => "House",
            Self::Techno => "Techno",
            Self::Electro => "Electro",
            Self::UkGarage => "UK Garage / 2-Step",
            Self::Breakbeat => "Breakbeat",
            Self::JungleDnb => "Jungle / DnB",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SongSection {
    Intro,
    Verse,
    Build,
    HighEnergy,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicrotimingPreset {
    Straight,
    Shuffle,
    LaidBack,
    Push,
    Skank,
    OneDrop,
    Broken,
}

#[allow(dead_code)]
impl MicrotimingPreset {
    pub const ALL: [Self; 7] = [
        Self::Straight,
        Self::Shuffle,
        Self::LaidBack,
        Self::Push,
        Self::Skank,
        Self::OneDrop,
        Self::Broken,
    ];
}

impl SongSection {
    pub const ALL: [Self; 4] = [Self::Intro, Self::Verse, Self::Build, Self::HighEnergy];
}

impl fmt::Display for SongSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Intro => "Intro",
            Self::Verse => "Verse",
            Self::Build => "Build",
            Self::HighEnergy => "High Energy",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone)]
pub struct TrackConfig {
    pub enabled: bool,
    pub locked: bool,
    pub probability: u8,
    pub drum_type: DrumType,
    pub style: Style,
    pub section: SongSection,
}

impl TrackConfig {
    pub fn default_for(index: usize) -> Self {
        let drum_type = match index {
            0 => DrumType::BassDrum,
            1 => DrumType::Snare,
            2 => DrumType::ClosedHat,
            3 => DrumType::OpenHat,
            4 => DrumType::HandClap,
            5 => DrumType::Cymbal,
            6 => DrumType::Rimshot,
            _ => DrumType::Shaker,
        };

        Self {
            enabled: true,
            locked: false,
            probability: 100,
            drum_type,
            style: Style::SourceLibrary,
            section: SongSection::Verse,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenerationOptions {
    pub density: u8,
    pub complexity: u8,
    pub fill_amount: u8,
    pub groove: u8,
    pub humanize: u8,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            density: 50,
            complexity: 50,
            fill_amount: 50,
            groove: 50,
            humanize: 15,
        }
    }
}

impl GenerationOptions {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 100;

    pub fn density_factor(self) -> f32 {
        factor(self.density, 0.35, 1.75)
    }

    pub fn density_label(self) -> &'static str {
        match self.density {
            0..=24 => "Sparse",
            25..=54 => "Balanced",
            55..=79 => "Dense",
            _ => "Busy",
        }
    }

    pub fn complexity_factor(self) -> f32 {
        factor(self.complexity, 0.25, 1.9)
    }

    pub fn fill_factor(self) -> f32 {
        factor(self.fill_amount, 0.0, 1.8)
    }

    pub fn phrase_variation_factor(self) -> f32 {
        factor(self.complexity, 0.0, 1.0)
    }

    pub fn anchor_strength(self) -> f32 {
        factor(100u8.saturating_sub(self.density), 0.65, 1.0)
    }

    pub fn optional_hit_probability(self) -> f32 {
        factor(self.density, 0.2, 1.0)
    }

    pub fn groove_factor(self) -> f32 {
        factor(self.groove, 0.0, 1.65)
    }

    pub fn humanize_factor(self) -> f32 {
        f32::from(self.humanize.clamp(Self::MIN, Self::MAX)) / 100.0
    }
}

fn factor(value: u8, min: f32, max: f32) -> f32 {
    let normalized = f32::from(value.clamp(GenerationOptions::MIN, GenerationOptions::MAX)) / 100.0;
    min + (max - min) * normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_options_defaults_are_normalized() {
        let options = GenerationOptions::default();

        assert!(options.density_factor() > 0.0);
        assert_eq!(options.density_label(), "Balanced");
        assert!(options.complexity_factor() > 0.0);
        assert!(options.groove_factor() > 0.0);
        assert!((0.0..=1.0).contains(&options.humanize_factor()));
    }

    #[test]
    fn density_labels_map_to_expected_ranges() {
        assert_eq!(
            GenerationOptions {
                density: 0,
                ..GenerationOptions::default()
            }
            .density_label(),
            "Sparse"
        );
        assert_eq!(
            GenerationOptions {
                density: 25,
                ..GenerationOptions::default()
            }
            .density_label(),
            "Balanced"
        );
        assert_eq!(
            GenerationOptions {
                density: 55,
                ..GenerationOptions::default()
            }
            .density_label(),
            "Dense"
        );
        assert_eq!(
            GenerationOptions {
                density: 80,
                ..GenerationOptions::default()
            }
            .density_label(),
            "Busy"
        );
    }

    #[test]
    fn timing_constants_are_internally_consistent() {
        assert_eq!(STEPS_PER_BAR, BEATS_PER_BAR * STEPS_PER_BEAT);
        assert_eq!(TICKS_PER_STEP, u32::from(PPQ) / STEPS_PER_BEAT);
        assert!(DEFAULT_NOTE_DURATION_TICKS < TICKS_PER_STEP);
    }

    #[test]
    fn bar_length_options_include_four_bars_first() {
        assert_eq!(
            BarLength::ALL,
            [BarLength::Four, BarLength::Eight, BarLength::Sixteen]
        );
        assert_eq!(BarLength::Four.bars(), 4);
    }
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub track: usize,
    pub drum_type: DrumType,
    pub tick: u32,
    pub duration: u32,
    pub velocity: u8,
    pub source: HitSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSource {
    SourcePattern,
    StyleRule,
    Ghost,
    Fill,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub bars: u32,
    pub ppq: u16,
    pub hits: Vec<Hit>,
}

impl Pattern {
    pub fn empty(bars: u32, ppq: u16) -> Self {
        Self {
            bars,
            ppq,
            hits: Vec::new(),
        }
    }
}
