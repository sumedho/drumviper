use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[10],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 7, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[2, 3, 7, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[5, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[2, 3, 7, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[5, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 9, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 6, 7, 8, 9, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[7],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 4, 7, 11, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[2, 8, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[6, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[8, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[6, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[14],
            },
        ],
    },
];
