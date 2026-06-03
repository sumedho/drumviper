use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13, 16],
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
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13],
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
                drum_type: DrumType::Snare,
                steps: &[2, 5, 8, 13, 16],
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
                drum_type: DrumType::Snare,
                steps: &[2, 5, 10, 13, 16],
            },
        ],
    },
];
