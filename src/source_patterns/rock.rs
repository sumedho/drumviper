use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 11],
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
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
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
                steps: &[1, 8, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[15],
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
                steps: &[1, 8, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
];
