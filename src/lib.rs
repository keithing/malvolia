mod data;
mod engine;
mod midi;

#[macro_use]
extern crate vst;

use engine::Engine;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};


struct Malvolia {
    sample_rate: f64,
    note_duration: f64,
    note: Option<u8>,
    note_velocity: f64,
    engine: Engine
}

impl Malvolia {

    /// Process an incoming midi event.
    ///
    /// The midi data is split up like so:
    ///
    /// `data[0]`: Contains the status and the channel. Source: [source]
    /// `data[1]`: Contains the supplemental data for the message - so, if this was a NoteOn then
    ///            this would contain the note.
    /// `data[2]`: Further supplemental data. Would be velocity in the case of a NoteOn message.
    ///
    /// [source]: http://www.midimountain.com/midi/midi_status.htm
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8, note_velocity: u8) {
        self.note_duration = 0.0;
        self.note = Some(note);
        self.note_velocity = note_velocity as f64;
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }
}


impl Default for Malvolia {
    fn default() -> Malvolia {
        Malvolia {
            sample_rate: 44100.0,
            note_duration: 0.0,
            note: None,
            note_velocity: 100.0,
            engine: Engine::new(44100.0)
        }
    }
}

impl Plugin for Malvolia {
    fn get_info(&self) -> Info {
        Info {
            name: "Malvolia".to_string(),
            vendor: "Illyria".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    // Supresses warning about match statment only having one arm
    #[allow(unknown_lints)]
    #[allow(unused_variables)]
    #[allow(single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, outputs) = buffer.split();
        let output_count = outputs.len();
        let mut signal: f64;
        for sample_idx in 0..samples {
            let gate = self.note.is_some();
            let mut note_duration = self.note_duration;
            let freq: Option<f64> = match self.note {
                Some(note) => Some(midi::midi_note_to_freq(&note)),
                None => None
            };
            signal = self.engine.saw_osc.step(freq);
            signal = self.engine.adsr.step(signal, gate);

            let output_sample = signal as f32;

            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = output_sample;
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(Malvolia);
