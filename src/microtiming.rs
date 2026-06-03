use crate::models::{
    DrumType, GenerationOptions, MicrotimingPreset, MICROTIMING_BROKEN_OFFSET_TICKS,
    MICROTIMING_LARGE_OFFSET_TICKS, MICROTIMING_MEDIUM_OFFSET_TICKS,
    MICROTIMING_ONE_DROP_OFFSET_TICKS, MICROTIMING_SHUFFLE_OFFSET_TICKS,
    MICROTIMING_SMALL_OFFSET_TICKS, STEPS_PER_BAR, TICKS_PER_STEP,
};

pub fn offset_for(
    preset: MicrotimingPreset,
    step: u8,
    drum_type: DrumType,
    options: &GenerationOptions,
) -> i32 {
    let raw = raw_offset_for(preset, step, drum_type);
    (raw as f32 * options.groove_factor()).round() as i32
}

pub fn apply_offset(base_tick: u32, offset: i32, bars: u32) -> u32 {
    let max_tick = bars
        .saturating_mul(STEPS_PER_BAR)
        .saturating_mul(TICKS_PER_STEP)
        .saturating_sub(1);

    (i64::from(base_tick) + i64::from(offset)).clamp(0, i64::from(max_tick)) as u32
}

fn raw_offset_for(preset: MicrotimingPreset, step: u8, drum_type: DrumType) -> i32 {
    match preset {
        MicrotimingPreset::Straight => 0,
        MicrotimingPreset::Shuffle => shuffle_offset(step),
        MicrotimingPreset::LaidBack => laid_back_offset(step, drum_type),
        MicrotimingPreset::Push => push_offset(step, drum_type),
        MicrotimingPreset::Skank => skank_offset(step, drum_type),
        MicrotimingPreset::OneDrop => one_drop_offset(step, drum_type),
        MicrotimingPreset::Broken => broken_offset(step, drum_type),
    }
}

fn shuffle_offset(step: u8) -> i32 {
    if step % 2 == 0 {
        MICROTIMING_SHUFFLE_OFFSET_TICKS
    } else {
        0
    }
}

fn laid_back_offset(step: u8, drum_type: DrumType) -> i32 {
    match drum_type {
        DrumType::Snare | DrumType::HandClap | DrumType::Rimshot => MICROTIMING_MEDIUM_OFFSET_TICKS,
        DrumType::ClosedHat | DrumType::Shaker if step % 2 == 0 => MICROTIMING_SMALL_OFFSET_TICKS,
        _ => 0,
    }
}

fn push_offset(step: u8, drum_type: DrumType) -> i32 {
    match drum_type {
        DrumType::BassDrum if matches!(step, 4 | 10 | 15) => -MICROTIMING_SMALL_OFFSET_TICKS,
        DrumType::ClosedHat | DrumType::Shaker if step % 2 == 1 => -MICROTIMING_SMALL_OFFSET_TICKS,
        _ => 0,
    }
}

fn skank_offset(step: u8, drum_type: DrumType) -> i32 {
    match drum_type {
        DrumType::OpenHat | DrumType::ClosedHat | DrumType::Shaker
            if matches!(step, 3 | 7 | 11 | 15) =>
        {
            MICROTIMING_LARGE_OFFSET_TICKS
        }
        DrumType::Snare | DrumType::HandClap if matches!(step, 5 | 13) => {
            MICROTIMING_SMALL_OFFSET_TICKS
        }
        _ => 0,
    }
}

fn one_drop_offset(step: u8, drum_type: DrumType) -> i32 {
    match drum_type {
        DrumType::BassDrum | DrumType::Snare | DrumType::HandClap if matches!(step, 9 | 10) => {
            MICROTIMING_ONE_DROP_OFFSET_TICKS
        }
        DrumType::ClosedHat | DrumType::Shaker if step % 2 == 0 => MICROTIMING_SMALL_OFFSET_TICKS,
        _ => 0,
    }
}

fn broken_offset(step: u8, drum_type: DrumType) -> i32 {
    match drum_type {
        DrumType::BassDrum if matches!(step, 3 | 7 | 11 | 15) => -MICROTIMING_BROKEN_OFFSET_TICKS,
        DrumType::Snare | DrumType::Rimshot if matches!(step, 5 | 8 | 13 | 16) => {
            MICROTIMING_BROKEN_OFFSET_TICKS
        }
        DrumType::ClosedHat | DrumType::Shaker if step % 2 == 0 => MICROTIMING_SMALL_OFFSET_TICKS,
        DrumType::ClosedHat | DrumType::Shaker => -MICROTIMING_SMALL_OFFSET_TICKS,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_returns_zero_offset() {
        assert_eq!(
            offset_for(
                MicrotimingPreset::Straight,
                2,
                DrumType::ClosedHat,
                &GenerationOptions::default()
            ),
            0
        );
    }

    #[test]
    fn shuffle_affects_even_subdivision_steps() {
        let options = GenerationOptions {
            groove: 100,
            variation: 100,
            ..GenerationOptions::default()
        };

        assert!(offset_for(MicrotimingPreset::Shuffle, 2, DrumType::ClosedHat, &options) > 0);
        assert_eq!(
            offset_for(MicrotimingPreset::Shuffle, 3, DrumType::ClosedHat, &options),
            0
        );
    }

    #[test]
    fn broken_affects_odd_and_even_steps() {
        let options = GenerationOptions {
            groove: 100,
            variation: 100,
            ..GenerationOptions::default()
        };

        assert_ne!(
            offset_for(MicrotimingPreset::Broken, 3, DrumType::BassDrum, &options),
            0
        );
        assert_ne!(
            offset_for(MicrotimingPreset::Broken, 2, DrumType::ClosedHat, &options),
            0
        );
    }

    #[test]
    fn zero_groove_produces_no_deterministic_offset() {
        let options = GenerationOptions {
            groove: 0,
            ..GenerationOptions::default()
        };

        assert_eq!(
            offset_for(MicrotimingPreset::Shuffle, 2, DrumType::ClosedHat, &options),
            0
        );
    }

    #[test]
    fn applying_offsets_stays_inside_pattern_bounds() {
        assert_eq!(apply_offset(0, -MICROTIMING_LARGE_OFFSET_TICKS, 8), 0);
        assert!(
            apply_offset(
                8 * STEPS_PER_BAR * TICKS_PER_STEP,
                MICROTIMING_LARGE_OFFSET_TICKS,
                8
            ) < 8 * STEPS_PER_BAR * TICKS_PER_STEP
        );
    }
}
