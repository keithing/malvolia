mod data;
mod engine;
mod midi;

#[macro_use]
extern crate vst;

use engine::Engine;
use midi::MidiController;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};


struct Malvolia {
    sample_rate: f64,
    engine: Engine,
    midi: MidiController
}

impl Default for Malvolia {
    fn default() -> Malvolia {
        Malvolia {
            sample_rate: 44100.0,
            engine: Engine::new(44100.0),
            midi: MidiController::new()
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
                Event::Midi(ev) => self.midi.process_event(ev.data),
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
        for sample_idx in 0..samples {
            let (signal_left, signal_right) = self.engine.process_sample(&self.midi);
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                if buf_idx.checked_rem(2) == Some(0) {
                    buff[sample_idx] = signal_left as f32;
                } else {
                    buff[sample_idx] = signal_right as f32;
                }
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
