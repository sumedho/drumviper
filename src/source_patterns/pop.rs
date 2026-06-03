use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 4, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 2, 4, 7, 8, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 7, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 7, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 5, 7, 9, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 4, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 4, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 2, 4, 7, 8, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 4, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 2, 4, 7, 8, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 4, 5, 8, 9, 12, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
];
