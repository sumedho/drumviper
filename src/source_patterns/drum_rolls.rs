use super::{SourcePattern, SourcePatternRow};
use crate::models::DrumType;

pub const PATTERNS: &[SourcePattern] = &[
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[11, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[9, 10, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[12, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::OpenHat,
                steps: &[1],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 3, 4, 5, 7, 9, 10, 11, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[SourcePatternRow {
            drum_type: DrumType::Snare,
            steps: &[1, 2, 3, 5, 6, 7, 9, 11, 13, 14, 15, 16],
        }],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[1, 7, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[4, 5, 8, 9, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 7, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[1],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[5, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[1],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[5, 9, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[13, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[14],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[4, 7, 8],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[11, 12, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 9, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[5, 9, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 8, 13],
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
                drum_type: DrumType::HighTom,
                steps: &[3],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[5, 11],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[13, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 7, 9, 10],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[7, 8, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 4],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[7, 8, 11, 12],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 5, 6, 9, 10, 13, 14],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[5, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[2, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[5, 8],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[11, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 4, 7, 10, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 6, 9, 12],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[3, 12, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[11, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[12, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[5, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[5, 7],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[6],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[2],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[6, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[14],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 9, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 4, 7, 8, 11, 12, 15, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[7, 8, 9, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[10, 11, 12, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[4, 5, 6, 7, 8, 9, 14, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[1, 2, 3, 16],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Accent,
                steps: &[],
            },
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[2, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[4, 8, 12, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 5, 9, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 7, 8, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 5, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::ClosedHat,
                steps: &[3, 9, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[5, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 4],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[9, 10],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 5, 7, 8, 11, 13, 14, 15, 16],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[5, 11],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 7],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 5, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[1, 7, 13],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 2, 7, 8, 13, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[3, 5, 9, 11, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[3, 5, 9, 11, 15],
            },
        ],
    },
    SourcePattern {
        rows: &[
            SourcePatternRow {
                drum_type: DrumType::HighTom,
                steps: &[3, 5],
            },
            SourcePatternRow {
                drum_type: DrumType::MediumTom,
                steps: &[7, 15],
            },
            SourcePatternRow {
                drum_type: DrumType::LowTom,
                steps: &[16],
            },
            SourcePatternRow {
                drum_type: DrumType::Snare,
                steps: &[1, 9, 11, 14],
            },
            SourcePatternRow {
                drum_type: DrumType::BassDrum,
                steps: &[2, 4, 6, 8, 10, 12, 13],
            },
            SourcePatternRow {
                drum_type: DrumType::Cymbal,
                steps: &[9],
            },
        ],
    },
];
