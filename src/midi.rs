use crate::models::{DrumType, Pattern};
use midly::num::{u15, u24, u28, u4, u7};
use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use std::fs::File;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum MidiExportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn export_midi(
    pattern: &Pattern,
    bpm: u16,
    path: impl AsRef<Path>,
) -> Result<(), MidiExportError> {
    let mut events = Vec::new();
    let tempo = 60_000_000u32 / u32::from(bpm.max(1));

    events.push(AbsoluteEvent {
        tick: 0,
        kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::new(tempo))),
    });
    events.push(AbsoluteEvent {
        tick: 0,
        kind: TrackEventKind::Meta(MetaMessage::TimeSignature(4, 2, 24, 8)),
    });

    for hit in &pattern.hits {
        if let Some(note) = note_for(hit.drum_type) {
            events.push(AbsoluteEvent {
                tick: hit.tick,
                kind: TrackEventKind::Midi {
                    channel: u4::new(9),
                    message: MidiMessage::NoteOn {
                        key: u7::new(note),
                        vel: u7::new(hit.velocity),
                    },
                },
            });
            events.push(AbsoluteEvent {
                tick: hit.tick + hit.duration,
                kind: TrackEventKind::Midi {
                    channel: u4::new(9),
                    message: MidiMessage::NoteOff {
                        key: u7::new(note),
                        vel: u7::new(0),
                    },
                },
            });
        }
    }

    let end_tick = u32::from(pattern.ppq) * 4 * pattern.bars;
    events.push(AbsoluteEvent {
        tick: end_tick,
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });
    events.sort_by_key(|event| event.tick);

    let mut last_tick = 0;
    let track: Vec<TrackEvent<'static>> = events
        .into_iter()
        .map(|event| {
            let delta = event.tick.saturating_sub(last_tick);
            last_tick = event.tick;
            TrackEvent {
                delta: u28::new(delta),
                kind: event.kind,
            }
        })
        .collect();

    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(u15::new(pattern.ppq)),
        },
        tracks: vec![track],
    };

    let mut file = File::create(path)?;
    smf.write_std(&mut file)?;
    Ok(())
}

pub fn note_for(drum_type: DrumType) -> Option<u8> {
    match drum_type {
        DrumType::BassDrum => Some(36),
        DrumType::Snare => Some(38),
        DrumType::Rimshot => Some(37),
        DrumType::HandClap => Some(39),
        DrumType::ClosedHat => Some(42),
        DrumType::OpenHat => Some(46),
        DrumType::Cymbal => Some(49),
        DrumType::LowTom => Some(45),
        DrumType::MediumTom => Some(47),
        DrumType::HighTom => Some(50),
        DrumType::Cowbell => Some(56),
        DrumType::Shaker => Some(82),
        DrumType::Accent => None,
    }
}

struct AbsoluteEvent {
    tick: u32,
    kind: TrackEventKind<'static>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Hit, HitSource, DEFAULT_NOTE_DURATION_TICKS, DEFAULT_TEMPO_BPM, FILL_NOTE_DURATION_TICKS,
        PPQ, TICKS_PER_STEP,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn maps_core_gm_drum_notes() {
        assert_eq!(note_for(DrumType::BassDrum), Some(36));
        assert_eq!(note_for(DrumType::Snare), Some(38));
        assert_eq!(note_for(DrumType::ClosedHat), Some(42));
        assert_eq!(note_for(DrumType::OpenHat), Some(46));
        assert_eq!(note_for(DrumType::Accent), None);
    }

    #[test]
    fn exports_parseable_single_track_midi_with_tempo() {
        let pattern = Pattern {
            bars: 8,
            ppq: PPQ,
            hits: vec![Hit {
                track: 0,
                drum_type: DrumType::BassDrum,
                tick: 0,
                duration: DEFAULT_NOTE_DURATION_TICKS,
                velocity: 100,
                source: HitSource::StyleRule,
            }],
        };
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock is valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("drumviper-test-{unique}.mid"));

        export_midi(&pattern, DEFAULT_TEMPO_BPM, &path).expect("MIDI export succeeds");
        let bytes = fs::read(&path).expect("MIDI file can be read");
        let parsed = Smf::parse(&bytes).expect("MIDI file parses");
        let _ = fs::remove_file(&path);

        assert_eq!(parsed.tracks.len(), 1);
        assert!(parsed.tracks[0]
            .iter()
            .any(|event| matches!(event.kind, TrackEventKind::Meta(MetaMessage::Tempo(_)))));
    }

    #[test]
    fn exports_ghost_notes_with_their_low_velocity_and_timing() {
        let pattern = Pattern {
            bars: 8,
            ppq: PPQ,
            hits: vec![Hit {
                track: 1,
                drum_type: DrumType::Snare,
                tick: TICKS_PER_STEP * 2,
                duration: FILL_NOTE_DURATION_TICKS + TICKS_PER_STEP / 8,
                velocity: 42,
                source: HitSource::Ghost,
            }],
        };
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock is valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("drumviper-ghost-test-{unique}.mid"));

        export_midi(&pattern, DEFAULT_TEMPO_BPM, &path).expect("MIDI export succeeds");
        let bytes = fs::read(&path).expect("MIDI file can be read");
        let parsed = Smf::parse(&bytes).expect("MIDI file parses");
        let _ = fs::remove_file(&path);

        let mut absolute_tick = 0;
        let ghost_note_on = parsed.tracks[0].iter().find_map(|event| {
            absolute_tick += event.delta.as_int();

            match event.kind {
                TrackEventKind::Midi {
                    channel,
                    message: MidiMessage::NoteOn { key, vel },
                } if key.as_int() == 38 && vel.as_int() > 0 => {
                    Some((absolute_tick, channel.as_int(), vel.as_int()))
                }
                _ => None,
            }
        });

        assert_eq!(ghost_note_on, Some((TICKS_PER_STEP * 2, 9, 42)));
    }

    #[test]
    fn exports_high_complexity_generated_pattern_as_parseable_midi() {
        let library =
            crate::patterns::SourcePatternLibrary::load_embedded().expect("embedded JSON parses");
        let mut tracks: Vec<_> = (0..crate::models::TRACK_COUNT)
            .map(crate::models::TrackConfig::default_for)
            .collect();
        tracks[0].style = crate::models::Style::Breakbeat;
        tracks[1].style = crate::models::Style::Breakbeat;
        let options = crate::models::GenerationOptions {
            density: 100,
            complexity: 100,
            fill_amount: 100,
            groove: 80,
            humanize: 100,
        };
        let pattern = crate::generator::generate_pattern(
            &library,
            &tracks,
            crate::models::BarLength::Eight,
            &options,
        );
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock is valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("drumviper-complex-test-{unique}.mid"));

        export_midi(&pattern, 150, &path).expect("MIDI export succeeds");
        let bytes = fs::read(&path).expect("MIDI file can be read");
        let parsed = Smf::parse(&bytes).expect("MIDI file parses");
        let _ = fs::remove_file(&path);

        assert_eq!(parsed.tracks.len(), 1);
    }
}
