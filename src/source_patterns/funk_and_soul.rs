use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13, 16],
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
                steps: &[1, 3, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[5],
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
                steps: &[8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Rimshot,
                steps: &[15],
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
                steps: &[2, 5, 8, 10, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 7, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 12, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[8, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 8, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 8, 11, 12],
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
                steps: &[1, 4, 7, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 9, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 11, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[5, 9, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[5],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[1, 3, 5, 7, 8, 9, 10, 11, 13, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 6, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 6, 7, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 8, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 5, 6, 7, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 8, 10, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 3, 6, 9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 10, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 7, 8, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 4, 6, 9, 10, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 6, 8, 11, 12, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 4, 7, 9, 10, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 5, 7, 8, 11, 12, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 6, 8, 10, 13, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 6, 12, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 11, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 6, 8],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 12, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 4, 11, 13, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 10, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 12, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[3, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 6],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[3, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 6, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 7, 8, 10, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 5, 8, 9, 11, 13, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 7, 8, 10, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 5, 8, 9, 11, 13, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[8, 9, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[2, 14, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[5, 13],
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
                drum_type: DrumType::Cymbal,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 8, 10, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 9, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 7, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[4, 6, 7, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 2, 9, 10],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 5, 8, 9, 10, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 8, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 6, 11, 14, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 6, 8, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 6, 8, 9, 11, 14, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[3, 5, 7, 8, 10, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 3, 5, 7, 8, 10, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[5, 8, 11, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 6, 8, 9, 11, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 6, 9, 10, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 8, 10, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 8, 9, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 7, 8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 8, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 7, 8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 4, 6, 9, 10, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[4, 7, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 6, 7, 9, 10, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 3, 5, 8, 10, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 6, 8, 10, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14, 16],
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
                drum_type: DrumType::Cymbal,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 8, 9, 11, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 6, 8, 10, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 5, 8, 11, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 8, 10, 11, 12, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 8, 10, 11, 12, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 8, 11, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cowbell,
                steps: &[1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 4, 8, 9, 11, 12, 14, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 6, 7, 9, 11, 12, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 13, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 6, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 7, 8, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 9, 11, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 7, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 7, 8, 10, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 9, 11, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 7, 9, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 9],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 9, 10, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 4, 5, 7, 9, 11, 12, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 4, 11, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 7, 8, 10, 13, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 5, 11, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 3, 5, 6, 7, 10, 11, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 8, 9, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 6, 10, 13, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 6, 8, 9, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 9, 11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 12, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 9, 12, 15],
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
                drum_type: DrumType::Cymbal,
                steps: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 12, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 9, 12, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 11, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 9, 11, 12, 14, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 12, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 5, 9],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 8, 10, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 4, 5, 6, 8, 10, 12, 13, 14, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 7, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[2, 3, 5, 6, 8, 9, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 8, 12, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[2, 3, 5, 6, 8, 9, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 8, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 4, 7, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[7, 10, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 3, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[9],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[2, 5, 6, 8, 10, 11, 12, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 9, 13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 5, 7, 9, 11, 13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1],
            },
            SourcePatternRow {
                drum_type: DrumType::HandClap,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[5, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 9, 10],
            },
        ],
    },
];
