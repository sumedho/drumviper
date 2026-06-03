use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[1, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16],
            },
        ],
    },
];
