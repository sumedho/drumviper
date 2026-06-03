use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 5, 9, 11, 12, 13, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[7, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 7, 11, 13],
            },
        ],
    },
];
