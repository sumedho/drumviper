use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 5, 8, 9, 12, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 4, 7, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 5, 8, 9, 12, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 4, 8, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 5, 8, 9, 12, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 4, 7, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[4, 7, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[3, 4, 7, 8, 11, 12, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 4, 7, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 7, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[3, 4, 7, 8, 11, 12, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 5, 7, 11, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 4, 7, 10, 11],
            },
        ],
    },
];
