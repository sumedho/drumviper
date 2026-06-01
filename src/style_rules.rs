use crate::models::{DrumType, MicrotimingPreset, SongSection, Style};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct StyleRule {
    pub preferred_sections: &'static [&'static str],
    pub anchor_steps: &'static [u8],
    pub accent_steps: &'static [u8],
    pub base_steps: &'static [u8],
    pub density: f32,
    pub ghost_chance: f32,
    pub roll_chance: f32,
    pub microtiming: MicrotimingPreset,
}

pub fn rule_for(style: Style, drum_type: DrumType, section: SongSection) -> StyleRule {
    let mut rule = match style {
        Style::SourceLibrary => source_rule(drum_type),
        Style::HipHop => hip_hop_rule(drum_type),
        Style::Trap => trap_rule(drum_type),
        Style::Drill => drill_rule(drum_type),
        Style::House => house_rule(drum_type),
        Style::Techno => techno_rule(drum_type),
        Style::Electro => electro_rule(drum_type),
        Style::UkGarage => uk_garage_rule(drum_type),
        Style::Breakbeat => breakbeat_rule(drum_type),
        Style::JungleDnb => jungle_dnb_rule(drum_type),
    };

    let multiplier = match section {
        SongSection::Intro => 0.55,
        SongSection::Verse => 1.0,
        SongSection::Build => 1.2,
        SongSection::HighEnergy => 1.45,
    };
    rule.density *= multiplier;
    rule.ghost_chance *= multiplier.min(1.4);
    rule.roll_chance *= multiplier.min(1.5);
    rule
}

pub fn velocity_for(section: SongSection, source_boost: u8) -> u8 {
    let base = match section {
        SongSection::Intro => 78,
        SongSection::Verse => 94,
        SongSection::Build => 101,
        SongSection::HighEnergy => 112,
    };
    (base + source_boost).min(127)
}

pub fn should_add<R: Rng + ?Sized>(rng: &mut R, density: f32) -> bool {
    rng.gen_bool(density.clamp(0.0, 1.0) as f64)
}

fn source_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &[],
        anchor_steps: fallback_anchors(drum_type),
        accent_steps: fallback_accents(drum_type),
        base_steps: fallback_steps(drum_type),
        density: 0.18,
        ghost_chance: 0.02,
        roll_chance: 0.01,
        microtiming: MicrotimingPreset::Straight,
    }
}

fn hip_hop_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Hip Hop", "Funk and Soul", "Basic Patterns"],
        anchor_steps: match drum_type {
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::BassDrum => &[1],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum | DrumType::Snare | DrumType::HandClap => &[1, 5, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 3, 7, 11, 15],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::ClosedHat => &[1, 3, 5, 7, 9, 11, 13, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.35,
        ghost_chance: 0.12,
        roll_chance: 0.03,
        microtiming: MicrotimingPreset::LaidBack,
    }
}

fn trap_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Hip Hop", "Miami Bass", "EDM"],
        anchor_steps: match drum_type {
            DrumType::Snare | DrumType::HandClap => &[9],
            DrumType::BassDrum => &[1],
            DrumType::ClosedHat => &[1, 5, 9, 13],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::Snare | DrumType::HandClap => &[9],
            DrumType::BassDrum => &[1, 10],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 7, 10, 15],
            DrumType::Snare | DrumType::HandClap => &[9],
            DrumType::ClosedHat => &[1, 3, 5, 7, 9, 11, 13, 15],
            DrumType::OpenHat => &[7, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.42,
        ghost_chance: 0.08,
        roll_chance: 0.22,
        microtiming: MicrotimingPreset::Push,
    }
}

fn drill_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Hip Hop", "Breaks", "EDM"],
        anchor_steps: match drum_type {
            DrumType::Snare | DrumType::HandClap => &[9],
            DrumType::BassDrum => &[1],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::Snare | DrumType::HandClap => &[9, 13],
            DrumType::BassDrum => &[1, 10],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 4, 10, 14],
            DrumType::Snare | DrumType::HandClap => &[9, 13],
            DrumType::ClosedHat => &[1, 3, 5, 8, 9, 11, 13, 16],
            DrumType::OpenHat => &[6, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.36,
        ghost_chance: 0.1,
        roll_chance: 0.26,
        microtiming: MicrotimingPreset::Broken,
    }
}

fn house_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["House", "EDM", "Basic Patterns"],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::OpenHat => &[3, 7, 11, 15],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::OpenHat => &[3, 7, 11, 15],
            DrumType::ClosedHat => &[1, 3, 5, 7, 9, 11, 13, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.32,
        ghost_chance: 0.04,
        roll_chance: 0.02,
        microtiming: MicrotimingPreset::Straight,
    }
}

fn techno_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["EDM", "House", "Electro"],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::OpenHat => &[3, 7, 11, 15],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::OpenHat => &[3, 11],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 5, 9, 13],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::OpenHat => &[3, 7, 11, 15],
            DrumType::ClosedHat | DrumType::Shaker => &[1, 2, 3, 5, 7, 9, 10, 11, 13, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.4,
        ghost_chance: 0.07,
        roll_chance: 0.06,
        microtiming: MicrotimingPreset::Straight,
    }
}

fn electro_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Electro", "EDM", "Breaks"],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 7, 15],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum | DrumType::Snare | DrumType::HandClap => &[1, 5, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 4, 7, 11, 15],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::ClosedHat => &[1, 3, 5, 8, 9, 11, 13, 16],
            DrumType::Rimshot | DrumType::Cowbell => &[4, 8, 12, 16],
            _ => fallback_steps(drum_type),
        },
        density: 0.34,
        ghost_chance: 0.05,
        roll_chance: 0.05,
        microtiming: MicrotimingPreset::Push,
    }
}

fn uk_garage_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Breaks", "House", "Funk and Soul"],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 7],
            DrumType::Snare | DrumType::HandClap => &[5, 13],
            DrumType::ClosedHat | DrumType::Shaker => &[2, 4, 6, 8, 10, 12, 14, 16],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum | DrumType::Snare | DrumType::HandClap => &[1, 5, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 7, 11],
            DrumType::Snare | DrumType::HandClap => &[5, 12, 13],
            DrumType::ClosedHat | DrumType::Shaker => &[2, 4, 6, 8, 10, 12, 14, 16],
            DrumType::OpenHat => &[4, 12],
            _ => fallback_steps(drum_type),
        },
        density: 0.36,
        ghost_chance: 0.16,
        roll_chance: 0.08,
        microtiming: MicrotimingPreset::Skank,
    }
}

fn breakbeat_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &[
            "Breaks",
            "Rolling Breaks",
            "Standard Breaks",
            "Funk and Soul",
        ],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 7, 11],
            DrumType::Snare | DrumType::Rimshot => &[5, 13],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum | DrumType::Snare | DrumType::Rimshot => &[1, 5, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 4, 7, 11, 15],
            DrumType::Snare | DrumType::Rimshot => &[5, 8, 13, 16],
            DrumType::ClosedHat | DrumType::Shaker => &[1, 3, 4, 5, 7, 9, 11, 12, 13, 15],
            DrumType::OpenHat | DrumType::Cymbal => &[3, 7, 11, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.48,
        ghost_chance: 0.22,
        roll_chance: 0.12,
        microtiming: MicrotimingPreset::Shuffle,
    }
}

fn jungle_dnb_rule(drum_type: DrumType) -> StyleRule {
    StyleRule {
        preferred_sections: &["Drum and Bass", "Breaks", "Rolling Breaks", "Drum Rolls"],
        anchor_steps: match drum_type {
            DrumType::BassDrum => &[1, 11],
            DrumType::Snare | DrumType::Rimshot => &[5, 13],
            DrumType::ClosedHat | DrumType::Shaker => &[1, 3, 5, 7, 9, 11, 13, 15],
            _ => fallback_anchors(drum_type),
        },
        accent_steps: match drum_type {
            DrumType::BassDrum | DrumType::Snare | DrumType::Rimshot => &[1, 5, 11, 13],
            _ => fallback_accents(drum_type),
        },
        base_steps: match drum_type {
            DrumType::BassDrum => &[1, 7, 11, 15],
            DrumType::Snare | DrumType::Rimshot => &[5, 9, 13],
            DrumType::ClosedHat | DrumType::Shaker => &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 15, 16],
            DrumType::OpenHat | DrumType::Cymbal => &[3, 7, 11, 15],
            _ => fallback_steps(drum_type),
        },
        density: 0.58,
        ghost_chance: 0.28,
        roll_chance: 0.18,
        microtiming: MicrotimingPreset::Broken,
    }
}

fn fallback_anchors(drum_type: DrumType) -> &'static [u8] {
    match drum_type {
        DrumType::BassDrum => &[1],
        DrumType::Snare | DrumType::HandClap | DrumType::Rimshot => &[5, 13],
        DrumType::ClosedHat | DrumType::Shaker => &[1, 5, 9, 13],
        DrumType::OpenHat | DrumType::Cymbal => &[3, 11],
        DrumType::Accent => &[1, 5, 9, 13],
        _ => &[],
    }
}

fn fallback_accents(drum_type: DrumType) -> &'static [u8] {
    match drum_type {
        DrumType::BassDrum | DrumType::Snare | DrumType::HandClap | DrumType::Rimshot => {
            &[1, 5, 9, 13]
        }
        DrumType::OpenHat | DrumType::Cymbal => &[3, 11, 15],
        DrumType::ClosedHat | DrumType::Shaker => &[1, 5, 9, 13],
        _ => &[1, 9],
    }
}

fn fallback_steps(drum_type: DrumType) -> &'static [u8] {
    match drum_type {
        DrumType::BassDrum => &[1, 9],
        DrumType::Snare | DrumType::HandClap | DrumType::Rimshot => &[5, 13],
        DrumType::ClosedHat | DrumType::Shaker => &[1, 3, 5, 7, 9, 11, 13, 15],
        DrumType::OpenHat | DrumType::Cymbal => &[3, 7, 11, 15],
        DrumType::LowTom | DrumType::MediumTom | DrumType::HighTom | DrumType::Cowbell => {
            &[4, 8, 12, 16]
        }
        DrumType::Accent => &[1, 5, 9, 13],
    }
}
