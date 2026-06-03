use crate::microtiming;
use crate::models::{
    BarLength, DrumType, GenerationOptions, Hit, HitSource, Pattern, SongSection, TrackConfig,
    ACCENT_LANE_VELOCITY_BOOST, ACCENT_STEP_VELOCITY_BOOST, DEFAULT_NOTE_DURATION_TICKS,
    FILL_NOTE_DURATION_TICKS, FILL_SUBDIVISION_VELOCITY_REDUCTION, FILL_VELOCITY_OFFSET,
    FINAL_BEAT_STEPS, GHOST_VELOCITY_OFFSET, MAX_HUMANIZE_TIMING_OFFSET_TICKS,
    MAX_HUMANIZE_VELOCITY_OFFSET, PHRASE_RESPONSE_VELOCITY_BOOST, PHRASE_START_VELOCITY_BOOST, PPQ,
    STEPS_PER_BAR, TICKS_PER_STEP, TURNAROUND_VELOCITY_BOOST,
};
use crate::patterns::{SourcePatternLibrary, SourceRow};
use crate::source_patterns::SourceSection;
use crate::style_rules::{rule_for, should_add, velocity_for};
use rand::{seq::SliceRandom, Rng};

pub fn generate_pattern(
    library: &SourcePatternLibrary,
    tracks: &[TrackConfig],
    length: BarLength,
    options: &GenerationOptions,
) -> Pattern {
    let mut rng = rand::thread_rng();
    let mut pattern = Pattern::empty(length.bars(), PPQ);

    for (index, track) in tracks.iter().enumerate() {
        if track.enabled {
            pattern.hits.extend(generate_track(
                library,
                index,
                track,
                length.bars(),
                options,
                &mut rng,
            ));
        }
    }

    if options.variation > 0 {
        apply_accents(&mut pattern.hits);
    }
    apply_humanize(&mut pattern.hits, length.bars(), options, &mut rng);
    pattern.hits.sort_by_key(|hit| (hit.tick, hit.track));
    pattern
}

pub fn generate_pattern_preserving_locked(
    library: &SourcePatternLibrary,
    existing: &Pattern,
    tracks: &[TrackConfig],
    length: BarLength,
    options: &GenerationOptions,
) -> Pattern {
    let mut rng = rand::thread_rng();
    let locked_hits: Vec<Hit> = if existing.bars == length.bars() {
        existing
            .hits
            .iter()
            .filter(|hit| tracks.get(hit.track).is_some_and(|track| track.locked))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    let mut generated_hits = Vec::new();

    for (index, track) in tracks.iter().enumerate() {
        if track.enabled && !track.locked {
            generated_hits.extend(generate_track(
                library,
                index,
                track,
                length.bars(),
                options,
                &mut rng,
            ));
        }
    }

    if options.variation > 0 {
        apply_accents(&mut generated_hits);
    }
    apply_humanize(&mut generated_hits, length.bars(), options, &mut rng);
    let mut hits = locked_hits;
    hits.extend(generated_hits);
    hits.sort_by_key(|hit| (hit.tick, hit.track));

    Pattern {
        bars: length.bars(),
        ppq: PPQ,
        hits,
    }
}

pub fn generate_single_track(
    library: &SourcePatternLibrary,
    existing: &Pattern,
    track_index: usize,
    track: &TrackConfig,
    length: BarLength,
    options: &GenerationOptions,
) -> Pattern {
    let mut rng = rand::thread_rng();
    let mut hits: Vec<Hit> = existing
        .hits
        .iter()
        .filter(|hit| hit.track != track_index)
        .cloned()
        .collect();
    let mut generated_hits = Vec::new();

    if track.enabled {
        generated_hits.extend(generate_track(
            library,
            track_index,
            track,
            length.bars(),
            options,
            &mut rng,
        ));
    }

    if options.variation > 0 {
        apply_accents(&mut generated_hits);
    }
    apply_humanize(&mut generated_hits, length.bars(), options, &mut rng);
    hits.extend(generated_hits);
    hits.sort_by_key(|hit| (hit.tick, hit.track));

    Pattern {
        bars: length.bars(),
        ppq: PPQ,
        hits,
    }
}

fn generate_track<R: Rng + ?Sized>(
    library: &SourcePatternLibrary,
    track_index: usize,
    track: &TrackConfig,
    bars: u32,
    options: &GenerationOptions,
    rng: &mut R,
) -> Vec<Hit> {
    if track.probability == 0 {
        return Vec::new();
    }

    if options.variation == 0 {
        if let Some(hits) = generate_exact_source_track(library, track_index, track, bars, rng) {
            return hits;
        }
    }

    let track_amount = track_amount_factor(track);
    let mut rule = rule_for(track.style, track.drum_type, track.section);
    let variation = options.variation_factor();
    rule.density *= options.density_factor() * track_amount * variation;
    rule.ghost_chance *= options.complexity_factor() * track_amount * variation;
    rule.roll_chance *= options.complexity_factor() * options.fill_factor() * track_amount;
    let mut hits = Vec::new();
    let source_rows = pick_source_rows(
        library,
        track.drum_type,
        track.style.source_section(),
        rule.preferred_sections,
        rng,
    );
    let source_keep_chance = source_keep_chance(options);

    for bar in 0..bars {
        let phrase = phrase_role(bar, bars);
        let source_row = source_rows.choose(rng);
        let source_steps = source_row.map(|row| row.steps.as_slice()).unwrap_or(&[]);

        for step in 1..=STEPS_PER_BAR as u8 {
            let from_anchor = is_anchor_hit(rule.anchor_steps, step) && anchor_allows(options, rng);
            let from_source = source_steps.contains(&step)
                && optional_hit_allows(source_keep_chance, phrase, options, track_amount, rng);
            let from_rule = rule.base_steps.contains(&step)
                && should_add(rng, phrase_density(rule.density, phrase, options));

            if from_anchor || from_source || from_rule {
                let source = if from_source {
                    HitSource::SourcePattern
                } else {
                    HitSource::StyleRule
                };
                hits.push(make_hit(
                    track_index,
                    track.drum_type,
                    bar,
                    step,
                    track.section,
                    source,
                    accent_velocity_boost(rule.accent_steps, step, phrase),
                    microtiming::offset_for(rule.microtiming, step, track.drum_type, options),
                    bars,
                ));
            }

            if can_ghost(track.drum_type)
                && rng.gen_bool(phrase_ghost_chance(rule.ghost_chance, phrase, options) as f64)
            {
                hits.push(make_hit(
                    track_index,
                    track.drum_type,
                    bar,
                    step,
                    track.section,
                    HitSource::Ghost,
                    GHOST_VELOCITY_OFFSET,
                    microtiming::offset_for(rule.microtiming, step, track.drum_type, options),
                    bars,
                ));
            }
        }

        if should_fill(track.section, bar, bars, phrase, options, track_amount) {
            add_fill(
                &mut hits,
                track_index,
                track.drum_type,
                bar,
                bars,
                track.section,
                rng,
                rule.roll_chance,
            );
        }
    }

    dedupe_hits(hits)
}

fn pick_source_rows<R: Rng + ?Sized>(
    library: &SourcePatternLibrary,
    drum_type: DrumType,
    source_section: Option<SourceSection>,
    preferred_sections: &[&str],
    rng: &mut R,
) -> Vec<SourceRow> {
    if let Some(section) = source_section {
        let section_rows = library.rows_for_section(section, drum_type);
        if !section_rows.is_empty() {
            return section_rows.choose_multiple(rng, 6).cloned().collect();
        }
    }

    let rows = library.rows_for(drum_type);
    let preferred: Vec<_> = rows
        .iter()
        .filter(|row| {
            preferred_sections
                .iter()
                .any(|section| row.section.label().contains(section))
        })
        .cloned()
        .collect();

    let pool: Vec<_> = if preferred.is_empty() {
        rows.to_vec()
    } else {
        preferred
    };

    pool.choose_multiple(rng, 6).cloned().collect()
}

fn generate_exact_source_track<R: Rng + ?Sized>(
    library: &SourcePatternLibrary,
    track_index: usize,
    track: &TrackConfig,
    bars: u32,
    rng: &mut R,
) -> Option<Vec<Hit>> {
    let rows = if let Some(section) = track.style.source_section() {
        let section_rows = library.rows_for_section(section, track.drum_type);
        if section_rows.is_empty() {
            library.rows_for(track.drum_type)
        } else {
            section_rows
        }
    } else if track.style == crate::models::Style::SourceLibrary {
        library.rows_for(track.drum_type)
    } else {
        return None;
    };

    let row = rows.choose(rng)?;
    let mut hits = Vec::new();

    for bar in 0..bars {
        for step in &row.steps {
            hits.push(make_hit(
                track_index,
                track.drum_type,
                bar,
                *step,
                track.section,
                HitSource::SourcePattern,
                0,
                0,
                bars,
            ));
        }
    }

    Some(dedupe_hits(hits))
}

fn make_hit(
    track: usize,
    drum_type: DrumType,
    bar: u32,
    step: u8,
    section: SongSection,
    source: HitSource,
    velocity_offset: i16,
    microtiming_offset: i32,
    bars: u32,
) -> Hit {
    let step_index = u32::from(step.saturating_sub(1));
    let base_tick = bar * STEPS_PER_BAR * TICKS_PER_STEP + step_index * TICKS_PER_STEP;
    let tick = microtiming::apply_offset(base_tick, microtiming_offset, bars);

    let source_boost = if source == HitSource::SourcePattern {
        4
    } else {
        0
    };
    let velocity =
        (i16::from(velocity_for(section, source_boost)) + velocity_offset).clamp(1, 127) as u8;

    Hit {
        track,
        drum_type,
        tick,
        duration: DEFAULT_NOTE_DURATION_TICKS,
        velocity,
        source,
    }
}

fn can_ghost(drum_type: DrumType) -> bool {
    matches!(
        drum_type,
        DrumType::Snare
            | DrumType::Rimshot
            | DrumType::ClosedHat
            | DrumType::Shaker
            | DrumType::HandClap
            | DrumType::BassDrum
    )
}

fn should_fill(
    section: SongSection,
    bar: u32,
    _bars: u32,
    phrase: PhraseRole,
    options: &GenerationOptions,
    track_amount: f32,
) -> bool {
    if options.variation == 0
        || options.fill_amount == 0
        || options.complexity < 15
        || track_amount <= 0.0
    {
        return false;
    }

    let fill_tier = fill_tier(options.fill_amount);

    match section {
        SongSection::Intro => false,
        SongSection::Verse => matches!(phrase, PhraseRole::Turnaround) && fill_tier >= 1,
        SongSection::Build => {
            if fill_tier >= 3 {
                matches!(phrase, PhraseRole::Response | PhraseRole::Turnaround)
            } else if fill_tier >= 2 {
                (bar + 1) % 2 == 0
            } else {
                matches!(phrase, PhraseRole::Turnaround)
            }
        }
        SongSection::HighEnergy => {
            if fill_tier >= 3 {
                !matches!(phrase, PhraseRole::Establish)
            } else if fill_tier >= 2 {
                matches!(phrase, PhraseRole::Response | PhraseRole::Turnaround)
            } else {
                matches!(phrase, PhraseRole::Turnaround)
            }
        }
    }
}

fn add_fill<R: Rng + ?Sized>(
    hits: &mut Vec<Hit>,
    track: usize,
    drum_type: DrumType,
    bar: u32,
    bars: u32,
    section: SongSection,
    rng: &mut R,
    roll_chance: f32,
) {
    if !matches!(
        drum_type,
        DrumType::Snare
            | DrumType::Rimshot
            | DrumType::ClosedHat
            | DrumType::OpenHat
            | DrumType::Cymbal
            | DrumType::Shaker
            | DrumType::HighTom
            | DrumType::MediumTom
            | DrumType::LowTom
    ) {
        return;
    }

    for step in FINAL_BEAT_STEPS {
        if rng.gen_bool(roll_chance.clamp(0.0, 1.0) as f64) {
            hits.push(make_hit(
                track,
                drum_type,
                bar,
                step,
                section,
                HitSource::Fill,
                FILL_VELOCITY_OFFSET,
                0,
                bars,
            ));

            if matches!(
                drum_type,
                DrumType::ClosedHat | DrumType::Snare | DrumType::Rimshot
            ) && rng.gen_bool(0.35)
            {
                let base =
                    bar * STEPS_PER_BAR * TICKS_PER_STEP + u32::from(step - 1) * TICKS_PER_STEP;
                hits.push(Hit {
                    track,
                    drum_type,
                    tick: base + TICKS_PER_STEP / 2,
                    duration: FILL_NOTE_DURATION_TICKS,
                    velocity: velocity_for(section, 0)
                        .saturating_sub(FILL_SUBDIVISION_VELOCITY_REDUCTION),
                    source: HitSource::Fill,
                });
            }
        }
    }
}

fn dedupe_hits(mut hits: Vec<Hit>) -> Vec<Hit> {
    hits.sort_by_key(|hit| (hit.tick, hit.track));
    let mut deduped: Vec<Hit> = Vec::new();

    for hit in hits {
        if let Some(existing) = deduped
            .iter_mut()
            .find(|existing| existing.tick == hit.tick && existing.track == hit.track)
        {
            if hit.velocity > existing.velocity {
                existing.velocity = hit.velocity;
                existing.source = hit.source;
            }
        } else {
            deduped.push(hit);
        }
    }

    deduped
}

fn apply_accents(hits: &mut [Hit]) {
    let accent_ticks: Vec<_> = hits
        .iter()
        .filter(|hit| hit.drum_type == DrumType::Accent)
        .map(|hit| hit.tick)
        .collect();

    if accent_ticks.is_empty() {
        return;
    }

    for hit in hits {
        if hit.drum_type != DrumType::Accent && accent_ticks.contains(&hit.tick) {
            hit.velocity = hit
                .velocity
                .saturating_add(ACCENT_LANE_VELOCITY_BOOST)
                .min(127);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PhraseRole {
    Establish,
    Response,
    Turnaround,
}

fn phrase_role(bar: u32, bars: u32) -> PhraseRole {
    if bar + 1 == bars || (bar + 1) % 8 == 0 {
        PhraseRole::Turnaround
    } else if bar % 4 >= 2 {
        PhraseRole::Response
    } else {
        PhraseRole::Establish
    }
}

fn is_anchor_hit(anchor_steps: &[u8], step: u8) -> bool {
    anchor_steps.contains(&step)
}

fn anchor_allows<R: Rng + ?Sized>(options: &GenerationOptions, rng: &mut R) -> bool {
    rng.gen_bool(options.anchor_strength().clamp(0.0, 1.0) as f64)
}

fn optional_hit_allows<R: Rng + ?Sized>(
    base_chance: f32,
    phrase: PhraseRole,
    options: &GenerationOptions,
    track_amount: f32,
    rng: &mut R,
) -> bool {
    let phrase_factor = match phrase {
        PhraseRole::Establish => 0.88,
        PhraseRole::Response => 0.96 + options.phrase_variation_factor() * 0.18,
        PhraseRole::Turnaround => 1.0 + options.phrase_variation_factor() * 0.3,
    };

    rng.gen_bool((base_chance * phrase_factor * track_amount).clamp(0.0, 1.0) as f64)
}

fn phrase_density(base_density: f32, phrase: PhraseRole, options: &GenerationOptions) -> f32 {
    let phrase_factor = match phrase {
        PhraseRole::Establish => 0.82,
        PhraseRole::Response => 0.92 + options.phrase_variation_factor() * 0.25,
        PhraseRole::Turnaround => 1.0 + options.phrase_variation_factor() * 0.45,
    };

    (base_density * phrase_factor * options.optional_hit_probability()).clamp(0.0, 1.0)
}

fn phrase_ghost_chance(base_chance: f32, phrase: PhraseRole, options: &GenerationOptions) -> f32 {
    let phrase_factor = match phrase {
        PhraseRole::Establish => 0.7,
        PhraseRole::Response => 1.0,
        PhraseRole::Turnaround => 1.2,
    };

    (base_chance * phrase_factor * options.complexity_factor()).clamp(0.0, 1.0)
}

fn accent_velocity_boost(accent_steps: &[u8], step: u8, phrase: PhraseRole) -> i16 {
    let step_boost = if accent_steps.contains(&step) {
        ACCENT_STEP_VELOCITY_BOOST
    } else {
        0
    };
    let phrase_boost = match phrase {
        PhraseRole::Establish if step == 1 => PHRASE_START_VELOCITY_BOOST,
        PhraseRole::Response if matches!(step, 5 | 13) => PHRASE_RESPONSE_VELOCITY_BOOST,
        PhraseRole::Turnaround if step >= 13 => TURNAROUND_VELOCITY_BOOST,
        _ => 0,
    };

    step_boost + phrase_boost
}

fn fill_tier(fill_amount: u8) -> u8 {
    match fill_amount {
        0 => 0,
        1..=34 => 1,
        35..=69 => 2,
        _ => 3,
    }
}

fn track_amount_factor(track: &TrackConfig) -> f32 {
    f32::from(track.probability.clamp(0, 100)) / 100.0
}

fn apply_humanize<R: Rng + ?Sized>(
    hits: &mut [Hit],
    bars: u32,
    options: &GenerationOptions,
    rng: &mut R,
) {
    let amount = options.humanize_factor();

    if amount <= 0.0 {
        return;
    }

    let max_tick = bars * STEPS_PER_BAR * TICKS_PER_STEP;
    let max_offset = (amount * MAX_HUMANIZE_TIMING_OFFSET_TICKS).round() as i32;
    let max_velocity_offset = (amount * MAX_HUMANIZE_VELOCITY_OFFSET).round() as i16;

    for hit in hits {
        if max_offset > 0 {
            let offset = rng.gen_range(-max_offset..=max_offset);
            hit.tick = (hit.tick as i64 + i64::from(offset))
                .clamp(0, i64::from(max_tick.saturating_sub(1))) as u32;
        }

        if max_velocity_offset > 0 {
            let velocity_offset = rng.gen_range(-max_velocity_offset..=max_velocity_offset);
            hit.velocity = (i16::from(hit.velocity) + velocity_offset).clamp(1, 127) as u8;
        }
    }
}

fn source_keep_chance(options: &GenerationOptions) -> f32 {
    (0.72 + options.density_factor() * 0.18).clamp(0.35, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BarLength, TrackConfig, TRACK_COUNT};
    use crate::patterns::SourcePatternLibrary;
    use rand::SeedableRng;

    #[test]
    fn generated_hits_stay_inside_selected_length() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        let pattern = generate_pattern(
            &library,
            &tracks,
            BarLength::Eight,
            &GenerationOptions::default(),
        );
        let end_tick = u32::from(pattern.ppq) * 4 * pattern.bars;

        assert_eq!(pattern.bars, 8);
        assert!(!pattern.hits.is_empty());
        assert!(pattern.hits.iter().all(|hit| hit.tick < end_tick));
    }

    #[test]
    fn preserving_locked_tracks_keeps_existing_locked_hits() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let mut tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        tracks[0].locked = true;
        let locked_hit = Hit {
            track: 0,
            drum_type: DrumType::BassDrum,
            tick: TICKS_PER_STEP,
            duration: DEFAULT_NOTE_DURATION_TICKS,
            velocity: 100,
            source: HitSource::StyleRule,
        };
        let existing = Pattern {
            bars: 8,
            ppq: PPQ,
            hits: vec![locked_hit.clone()],
        };

        let pattern = generate_pattern_preserving_locked(
            &library,
            &existing,
            &tracks,
            BarLength::Eight,
            &GenerationOptions::default(),
        );

        assert!(pattern.hits.iter().any(|hit| {
            hit.track == locked_hit.track
                && hit.tick == locked_hit.tick
                && hit.velocity == locked_hit.velocity
                && hit.drum_type == locked_hit.drum_type
        }));
    }

    #[test]
    fn humanize_keeps_hits_inside_pattern_bounds() {
        let mut hits = vec![
            Hit {
                track: 0,
                drum_type: DrumType::BassDrum,
                tick: 0,
                duration: DEFAULT_NOTE_DURATION_TICKS,
                velocity: 1,
                source: HitSource::StyleRule,
            },
            Hit {
                track: 1,
                drum_type: DrumType::Snare,
                tick: 8 * STEPS_PER_BAR * TICKS_PER_STEP - 1,
                duration: DEFAULT_NOTE_DURATION_TICKS,
                velocity: 127,
                source: HitSource::Ghost,
            },
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        let options = GenerationOptions {
            humanize: 100,
            variation: 100,
            ..GenerationOptions::default()
        };

        apply_humanize(&mut hits, 8, &options, &mut rng);

        let end_tick = 8 * STEPS_PER_BAR * TICKS_PER_STEP;
        assert!(hits.iter().all(|hit| hit.tick < end_tick));
        assert!(hits.iter().all(|hit| (1..=127).contains(&hit.velocity)));
    }

    #[test]
    fn density_and_complexity_extremes_generate_valid_patterns() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();

        for options in [
            GenerationOptions {
                density: 0,
                complexity: 0,
                fill_amount: 0,
                variation: 0,
                groove: 0,
                humanize: 0,
            },
            GenerationOptions {
                density: 100,
                complexity: 100,
                fill_amount: 100,
                variation: 100,
                groove: 100,
                humanize: 100,
            },
        ] {
            let pattern = generate_pattern(&library, &tracks, BarLength::Eight, &options);
            let end_tick = u32::from(pattern.ppq) * 4 * pattern.bars;

            assert_eq!(pattern.bars, 8);
            assert!(pattern.hits.iter().all(|hit| hit.tick < end_tick));
            assert!(pattern
                .hits
                .iter()
                .all(|hit| (1..=127).contains(&hit.velocity)));
        }
    }

    #[test]
    fn track_probability_zero_prevents_generated_hits_for_track() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let mut track = TrackConfig::default_for(0);
        track.probability = 0;
        let pattern = generate_single_track(
            &library,
            &Pattern::empty(8, PPQ),
            0,
            &track,
            BarLength::Eight,
            &GenerationOptions::default(),
        );

        assert!(pattern.hits.iter().all(|hit| hit.track != 0));
    }

    #[test]
    fn fill_amount_zero_prevents_fill_hits() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let mut tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        tracks[1].style = crate::models::Style::Breakbeat;
        tracks[1].section = SongSection::HighEnergy;
        let options = GenerationOptions {
            fill_amount: 0,
            complexity: 100,
            ..GenerationOptions::default()
        };

        let pattern = generate_pattern(&library, &tracks, BarLength::Eight, &options);

        assert!(!pattern.hits.iter().any(|hit| hit.source == HitSource::Fill));
    }

    #[test]
    fn zero_variation_repeats_exact_source_section_row() {
        let library = SourcePatternLibrary::load_embedded().expect("static source patterns load");
        let track = TrackConfig {
            drum_type: DrumType::BassDrum,
            style: crate::models::Style::Source(
                crate::source_patterns::SourceSection::StandardBreaks,
            ),
            section: SongSection::Verse,
            ..TrackConfig::default_for(0)
        };
        let options = GenerationOptions {
            fill_amount: 100,
            humanize: 100,
            variation: 0,
            ..GenerationOptions::default()
        };

        let pattern = generate_single_track(
            &library,
            &Pattern::empty(4, PPQ),
            0,
            &track,
            BarLength::Four,
            &options,
        );
        let section_rows = library.rows_for_section(
            crate::source_patterns::SourceSection::StandardBreaks,
            DrumType::BassDrum,
        );
        let first_bar_steps: Vec<_> = pattern
            .hits
            .iter()
            .filter(|hit| hit.track == 0 && hit.tick < STEPS_PER_BAR * TICKS_PER_STEP)
            .map(|hit| (hit.tick / TICKS_PER_STEP + 1) as u8)
            .collect();

        assert!(section_rows
            .iter()
            .any(|row| row.steps.as_slice() == first_bar_steps.as_slice()));
        assert!(pattern
            .hits
            .iter()
            .all(|hit| hit.source == HitSource::SourcePattern));
        assert!(!pattern
            .hits
            .iter()
            .any(|hit| hit.source == HitSource::Ghost || hit.source == HitSource::Fill));

        for bar in 0..4 {
            let bar_start = bar * STEPS_PER_BAR * TICKS_PER_STEP;
            let bar_steps: Vec<_> = pattern
                .hits
                .iter()
                .filter(|hit| {
                    hit.track == 0
                        && hit.tick >= bar_start
                        && hit.tick < bar_start + STEPS_PER_BAR * TICKS_PER_STEP
                })
                .map(|hit| ((hit.tick - bar_start) / TICKS_PER_STEP + 1) as u8)
                .collect();
            assert_eq!(bar_steps, first_bar_steps);
        }
    }

    #[test]
    fn high_fill_amount_enables_turnaround_fills() {
        let options = GenerationOptions {
            fill_amount: 100,
            complexity: 100,
            variation: 100,
            ..GenerationOptions::default()
        };

        assert!(should_fill(
            SongSection::HighEnergy,
            7,
            8,
            PhraseRole::Turnaround,
            &options,
            1.0
        ));
        assert!(should_fill(
            SongSection::Build,
            3,
            8,
            PhraseRole::Response,
            &options,
            1.0
        ));
    }

    #[test]
    fn low_nonzero_track_amount_preserves_anchors() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let track = TrackConfig {
            drum_type: DrumType::BassDrum,
            style: crate::models::Style::House,
            section: SongSection::Verse,
            probability: 1,
            ..TrackConfig::default_for(0)
        };
        let options = GenerationOptions {
            density: 0,
            humanize: 0,
            ..GenerationOptions::default()
        };

        let pattern = generate_single_track(
            &library,
            &Pattern::empty(8, PPQ),
            0,
            &track,
            BarLength::Eight,
            &options,
        );

        assert!(pattern.hits.iter().any(|hit| hit.track == 0));
        assert!(pattern
            .hits
            .iter()
            .any(|hit| hit.track == 0 && hit.tick == 0));
    }

    #[test]
    fn global_generation_respects_each_track_amount() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let mut tracks: Vec<_> = (0..TRACK_COUNT).map(TrackConfig::default_for).collect();
        tracks[0].probability = 0;
        tracks[1].probability = 1;
        tracks[1].drum_type = DrumType::Snare;
        tracks[1].style = crate::models::Style::HipHop;

        let pattern = generate_pattern_preserving_locked(
            &library,
            &Pattern::empty(8, PPQ),
            &tracks,
            BarLength::Eight,
            &GenerationOptions {
                density: 0,
                humanize: 0,
                ..GenerationOptions::default()
            },
        );

        assert!(pattern.hits.iter().all(|hit| hit.track != 0));
        assert!(pattern.hits.iter().any(|hit| hit.track == 1));
    }

    #[test]
    fn low_density_preserves_house_kick_anchors() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let track = TrackConfig {
            drum_type: DrumType::BassDrum,
            style: crate::models::Style::House,
            section: SongSection::Verse,
            ..TrackConfig::default_for(0)
        };
        let options = GenerationOptions {
            density: 0,
            humanize: 0,
            ..GenerationOptions::default()
        };

        let pattern = generate_single_track(
            &library,
            &Pattern::empty(8, PPQ),
            0,
            &track,
            BarLength::Eight,
            &options,
        );
        let expected_ticks = [
            0,
            TICKS_PER_STEP * 4,
            TICKS_PER_STEP * 8,
            TICKS_PER_STEP * 12,
        ];

        for tick in expected_ticks {
            assert!(pattern
                .hits
                .iter()
                .any(|hit| hit.track == 0 && hit.tick == tick));
        }
    }

    #[test]
    fn accent_boosts_increase_velocity_without_duplicate_same_track_hits() {
        let library = SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let track = TrackConfig {
            drum_type: DrumType::Snare,
            style: crate::models::Style::HipHop,
            section: SongSection::Verse,
            ..TrackConfig::default_for(1)
        };
        let options = GenerationOptions {
            density: 0,
            humanize: 0,
            ..GenerationOptions::default()
        };

        let pattern = generate_single_track(
            &library,
            &Pattern::empty(8, PPQ),
            1,
            &track,
            BarLength::Eight,
            &options,
        );
        let mut seen = std::collections::HashSet::new();

        for hit in pattern.hits.iter().filter(|hit| hit.track == 1) {
            assert!(seen.insert(hit.tick));
        }
        assert!(pattern
            .hits
            .iter()
            .any(|hit| hit.track == 1 && hit.velocity > 100));
    }
}
