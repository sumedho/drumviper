use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1],
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
                steps: &[1, 5, 9, 13],
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
                steps: &[3, 6, 11, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, 16],
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
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[1, 2, 3, 5, 7, 8, 9, 10, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[2, 4, 6, 8, 10, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 5, 9, 11, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[3, 5, 9, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[11, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[2, 8, 10],
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
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[2, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[3, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[4, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 7, 11, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3, 4, 7, 8, 10, 11, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 7, 8, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
];
