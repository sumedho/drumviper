use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[1, 5, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 10, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 7, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[7, 10, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[1, 5, 11, 13],
            },
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
                drum_type: DrumType::Accent,
                steps: &[3, 5, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 10, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[1, 5, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[1, 5, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[9, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 7, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11],
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
                drum_type: DrumType::Accent,
                steps: &[5, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 9, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[8, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[8, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 7, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11],
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
                drum_type: DrumType::BassDrum,
                steps: &[3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 10, 13],
            },
        ],
    },
];
