mod afro_cuban;
mod basic_patterns;
mod breaks;
mod breaks_kick;
mod breaks_snare;
mod drum_and_bass;
mod drum_rolls;
mod dub;
mod edm;
mod electro;
mod funk_and_soul;
mod ghost_snares;
mod hip_hop;
mod house;
mod hybrid_breaks_short;
mod hybrid_breaks_with_alternate_endings;
mod irregular_breaks;
mod miami_bass;
mod pop;
mod reggaeton;
mod rock;
mod rolling_breaks;
mod standard_breaks;

use crate::models::DrumType;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceSection {
    BasicPatterns,
    StandardBreaks,
    Rock,
    Electro,
    House,
    MiamiBass,
    HipHop,
    FunkAndSoul,
    AfroCuban,
    DrumAndBass,
    Edm,
    Dub,
    Reggaeton,
    Pop,
    Breaks,
    HybridBreaksWithAlternateEndings,
    HybridBreaksShort,
    IrregularBreaks,
    RollingBreaks,
    BreaksSnare,
    GhostSnares,
    BreaksKick,
    DrumRolls,
}

#[derive(Debug, Clone, Copy)]
pub struct SourcePattern {
    pub rows: &'static [SourcePatternRow],
}

#[derive(Debug, Clone, Copy)]
pub struct SourcePatternRow {
    pub drum_type: DrumType,
    pub steps: &'static [u8],
}

impl SourceSection {
    pub const ALL: [Self; 23] = [
        Self::BasicPatterns,
        Self::StandardBreaks,
        Self::Rock,
        Self::Electro,
        Self::House,
        Self::MiamiBass,
        Self::HipHop,
        Self::FunkAndSoul,
        Self::AfroCuban,
        Self::DrumAndBass,
        Self::Edm,
        Self::Dub,
        Self::Reggaeton,
        Self::Pop,
        Self::Breaks,
        Self::HybridBreaksWithAlternateEndings,
        Self::HybridBreaksShort,
        Self::IrregularBreaks,
        Self::RollingBreaks,
        Self::BreaksSnare,
        Self::GhostSnares,
        Self::BreaksKick,
        Self::DrumRolls,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::BasicPatterns => "Basic Patterns",
            Self::StandardBreaks => "Standard Breaks",
            Self::Rock => "Rock",
            Self::Electro => "Electro",
            Self::House => "House",
            Self::MiamiBass => "Miami Bass",
            Self::HipHop => "Hip Hop",
            Self::FunkAndSoul => "Funk and Soul",
            Self::AfroCuban => "Afro-Cuban",
            Self::DrumAndBass => "Drum and Bass",
            Self::Edm => "EDM",
            Self::Dub => "Dub",
            Self::Reggaeton => "Reggaeton",
            Self::Pop => "Pop",
            Self::Breaks => "Breaks",
            Self::HybridBreaksWithAlternateEndings => "Hybrid Breaks With Alternate Endings",
            Self::HybridBreaksShort => "Hybrid Breaks…",
            Self::IrregularBreaks => "Irregular Breaks",
            Self::RollingBreaks => "Rolling Breaks",
            Self::BreaksSnare => "Breaks - Snare",
            Self::GhostSnares => "Ghost Snares",
            Self::BreaksKick => "Breaks - Kick",
            Self::DrumRolls => "Drum Rolls",
        }
    }

    pub fn patterns(self) -> &'static [SourcePattern] {
        match self {
            Self::BasicPatterns => basic_patterns::PATTERNS,
            Self::StandardBreaks => standard_breaks::PATTERNS,
            Self::Rock => rock::PATTERNS,
            Self::Electro => electro::PATTERNS,
            Self::House => house::PATTERNS,
            Self::MiamiBass => miami_bass::PATTERNS,
            Self::HipHop => hip_hop::PATTERNS,
            Self::FunkAndSoul => funk_and_soul::PATTERNS,
            Self::AfroCuban => afro_cuban::PATTERNS,
            Self::DrumAndBass => drum_and_bass::PATTERNS,
            Self::Edm => edm::PATTERNS,
            Self::Dub => dub::PATTERNS,
            Self::Reggaeton => reggaeton::PATTERNS,
            Self::Pop => pop::PATTERNS,
            Self::Breaks => breaks::PATTERNS,
            Self::HybridBreaksWithAlternateEndings => {
                hybrid_breaks_with_alternate_endings::PATTERNS
            }
            Self::HybridBreaksShort => hybrid_breaks_short::PATTERNS,
            Self::IrregularBreaks => irregular_breaks::PATTERNS,
            Self::RollingBreaks => rolling_breaks::PATTERNS,
            Self::BreaksSnare => breaks_snare::PATTERNS,
            Self::GhostSnares => ghost_snares::PATTERNS,
            Self::BreaksKick => breaks_kick::PATTERNS,
            Self::DrumRolls => drum_rolls::PATTERNS,
        }
    }
}

impl fmt::Display for SourceSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
