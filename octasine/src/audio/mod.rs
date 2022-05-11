pub mod gen;
pub mod parameters;
pub mod voices;

use std::collections::VecDeque;

use array_init::array_init;
use fastrand::Rng;

use gen::VoiceData;
use vst::event::MidiEvent;

use crate::common::*;

use parameters::*;
use voices::*;

use self::voices::log10_table::Log10Table;

#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(pub f64);

impl Default for InterpolationDuration {
    fn default() -> Self {
        Self::slow()
    }
}

impl InterpolationDuration {
    /// 0.52 ms. 22.96875 samples with 44.1 kHz, 25 with 48 kHz
    fn fast() -> Self {
        Self(1.0 / 1920.0)
    }
    /// 1.04 ms. 45.9375 samples with 44.1 Hz, 50 with 48 kHz
    fn medium() -> Self {
        Self(1.0 / 960.0)
    }
    /// 3.33 ms. 147 samples with 44.1 Hz, 160 with 48 kHz
    fn slow() -> Self {
        Self(1.0 / 300.0)
    }
}

pub struct AudioState {
    sample_rate: SampleRate,
    time_per_sample: TimePerSample,
    pub bpm: BeatsPerMinute,
    pub parameters: AudioParameters,
    rng: Rng,
    log10table: Log10Table,
    voices: [Voice; 128],
    pending_midi_events: VecDeque<MidiEvent>,
    audio_gen_voice_data: [VoiceData; 128],
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            sample_rate: SampleRate::default(),
            time_per_sample: SampleRate::default().into(),
            bpm: Default::default(),
            parameters: AudioParameters::default(),
            rng: Rng::new(),
            log10table: Default::default(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            // Start with some capacity to cut down on later allocations
            pending_midi_events: VecDeque::with_capacity(128),
            audio_gen_voice_data: array_init::array_init(|_| VoiceData::default()),
        }
    }
}

impl AudioState {
    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        self.sample_rate = sample_rate;
        self.time_per_sample = sample_rate.into();
    }

    pub fn enqueue_midi_events<I: Iterator<Item = MidiEvent>>(&mut self, events: I) {
        for event in events {
            self.pending_midi_events.push_back(event);
        }

        self.pending_midi_events
            .make_contiguous()
            .sort_by_key(|e| e.delta_frames);
    }

    fn process_events_for_sample(&mut self, buffer_offset: usize) {
        loop {
            match self
                .pending_midi_events
                .get(0)
                .map(|e| e.delta_frames as usize)
            {
                Some(event_delta_frames) if event_delta_frames == buffer_offset => {
                    let event = self.pending_midi_events.pop_front().unwrap();

                    self.process_midi_event(event);
                }
                _ => break,
            }
        }
    }

    fn process_midi_event(&mut self, mut event: MidiEvent) {
        event.data[0] >>= 4;

        match event.data {
            [0b_1000, pitch, _] => self.key_off(pitch),
            [0b_1001, pitch, 0] => self.key_off(pitch),
            [0b_1001, pitch, velocity] => self.key_on(pitch, velocity),
            _ => (),
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
        self.voices[pitch as usize].press_key(velocity);
    }

    fn key_off(&mut self, pitch: u8) {
        self.voices[pitch as usize].release_key();
    }
}
