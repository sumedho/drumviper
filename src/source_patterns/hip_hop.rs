use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 8, 12, 15],
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
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 10, 14, 16],
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
                steps: &[1, 8, 9, 12],
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
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 9, 11],
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
                steps: &[1, 3, 9, 10, 12],
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
                steps: &[1, 4, 8, 10, 11, 16],
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
                steps: &[1, 3, 8, 9, 10, 11],
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
                steps: &[1, 3, 8, 9, 11, 16],
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
                steps: &[1, 3, 11, 12, 16],
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
                steps: &[1, 8, 11, 14, 16],
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
                steps: &[1, 4, 9, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 4, 5, 7, 8, 9, 10, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[6, 14],
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
                steps: &[9],
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
                steps: &[3, 5],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 5, 7, 9, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 3, 5, 7, 8, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 3, 5, 7, 8, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 8, 11, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
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
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 7, 9, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[1, 5, 7, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[3, 6, 9, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1, 4, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[8],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[8, 11, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[10, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[12, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Shaker,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 6, 10, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 9, 10, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
        ],
    },
];
