use crate::models::DrumType;
use serde::Deserialize;
use std::collections::HashMap;

const POCKET_OPERATIONS_JSON: &str = include_str!("../patterns/pocket_operations_patterns.json");

#[derive(Debug, Clone)]
pub struct SourcePatternLibrary {
    by_instrument: HashMap<DrumType, Vec<SourceRow>>,
    pattern_count: usize,
}

#[derive(Debug, Clone)]
pub struct SourceRow {
    pub section: String,
    pub steps: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct RawLibrary {
    patterns: Vec<RawPattern>,
}

#[derive(Debug, Deserialize)]
struct RawPattern {
    section: String,
    rows: HashMap<String, Vec<u8>>,
}

impl SourcePatternLibrary {
    pub fn empty() -> Self {
        Self {
            by_instrument: HashMap::new(),
            pattern_count: 0,
        }
    }

    pub fn load_embedded() -> Result<Self, serde_json::Error> {
        let raw: RawLibrary = serde_json::from_str(POCKET_OPERATIONS_JSON)?;
        let pattern_count = raw.patterns.len();
        let mut by_instrument: HashMap<DrumType, Vec<SourceRow>> = HashMap::new();

        for pattern in raw.patterns {
            for (code, steps) in pattern.rows {
                if let Some(drum_type) = DrumType::from_code(&code) {
                    by_instrument.entry(drum_type).or_default().push(SourceRow {
                        section: pattern.section.clone(),
                        steps,
                    });
                }
            }
        }

        Ok(Self {
            by_instrument,
            pattern_count,
        })
    }

    pub fn rows_for(&self, drum_type: DrumType) -> &[SourceRow] {
        self.by_instrument
            .get(&drum_type)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn instrument_count(&self, drum_type: DrumType) -> usize {
        self.rows_for(drum_type).len()
    }

    pub fn pattern_count(&self) -> usize {
        self.pattern_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_library_by_instrument() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");

        assert!(library.instrument_count(DrumType::BassDrum) > 200);
        assert!(library.instrument_count(DrumType::Snare) > 200);
        assert!(library.instrument_count(DrumType::ClosedHat) > 50);
    }
}
