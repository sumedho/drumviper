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
                steps: &[4, 7, 12, 15],
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
                drum_type: DrumType::Accent,
                steps: &[6, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[4, 6, 7, 8, 11, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 4, 7, 9, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9],
            },
        ],
    },
];
