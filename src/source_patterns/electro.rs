use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 12, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 9, 10],
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
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[5, 13],
            },
        ],
    },
];
