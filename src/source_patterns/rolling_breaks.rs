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
                steps: &[1, 11, 14],
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
                steps: &[1, 7, 13],
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
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11, 15],
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
                steps: &[1, 2],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 10, 11],
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
                steps: &[5, 13],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 11],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 12],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 8, 11, 14],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 12],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 11],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 11],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 11],
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
                steps: &[1, 8],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 11],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 9, 11],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 12],
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
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 6, 7, 10, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
        ],
    },
];
