use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 4, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 13, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[4, 13],
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
                steps: &[5, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 9, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 11, 14],
            },
        ],
    },
];
