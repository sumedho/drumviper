use crate::models::DrumType;
use crate::source_patterns::SourceSection;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SourcePatternLibrary {
    by_instrument: HashMap<DrumType, Vec<SourceRow>>,
    by_section_instrument: HashMap<(SourceSection, DrumType), Vec<SourceRow>>,
    pattern_count: usize,
}

#[derive(Debug, Clone)]
pub struct SourceRow {
    pub section: SourceSection,
    pub steps: Vec<u8>,
}

#[derive(Debug)]
pub struct SourcePatternLoadError;

impl fmt::Display for SourcePatternLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("static source pattern data failed to load")
    }
}

impl std::error::Error for SourcePatternLoadError {}

impl SourcePatternLibrary {
    pub fn empty() -> Self {
        Self {
            by_instrument: HashMap::new(),
            by_section_instrument: HashMap::new(),
            pattern_count: 0,
        }
    }

    pub fn load_embedded() -> Result<Self, SourcePatternLoadError> {
        let mut by_instrument: HashMap<DrumType, Vec<SourceRow>> = HashMap::new();
        let mut by_section_instrument: HashMap<(SourceSection, DrumType), Vec<SourceRow>> =
            HashMap::new();
        let mut pattern_count = 0;

        for section in SourceSection::ALL {
            for pattern in section.patterns() {
                pattern_count += 1;

                for row in pattern.rows {
                    let source_row = SourceRow {
                        section,
                        steps: row.steps.to_vec(),
                    };

                    by_instrument
                        .entry(row.drum_type)
                        .or_default()
                        .push(source_row.clone());
                    by_section_instrument
                        .entry((section, row.drum_type))
                        .or_default()
                        .push(source_row);
                }
            }
        }

        Ok(Self {
            by_instrument,
            by_section_instrument,
            pattern_count,
        })
    }

    pub fn rows_for(&self, drum_type: DrumType) -> &[SourceRow] {
        self.by_instrument
            .get(&drum_type)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn rows_for_section(&self, section: SourceSection, drum_type: DrumType) -> &[SourceRow] {
        self.by_section_instrument
            .get(&(section, drum_type))
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
    fn loads_static_library_by_instrument() {
        let library = SourcePatternLibrary::load_embedded().expect("static source patterns load");

        assert_eq!(library.pattern_count(), 269);
        assert!(library.instrument_count(DrumType::BassDrum) > 200);
        assert!(library.instrument_count(DrumType::Snare) > 200);
        assert!(library.instrument_count(DrumType::ClosedHat) > 50);
    }

    #[test]
    fn filters_rows_by_exact_source_section() {
        let library = SourcePatternLibrary::load_embedded().expect("static source patterns load");
        let rows = library.rows_for_section(SourceSection::StandardBreaks, DrumType::BassDrum);

        assert_eq!(rows.len(), 4);
        assert!(rows
            .iter()
            .all(|row| row.section == SourceSection::StandardBreaks));
    }
}
