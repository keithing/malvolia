mod osc;
mod envelope;

use midi::MidiController;

pub struct Voice {
    pub adsr: envelope::ADSR,
    pub osc: osc::WaveTable
}

impl Voice {
    pub fn new(sample_rate: f64) -> Voice {
        Voice {
            adsr: envelope::ADSR::new(sample_rate),
            osc: osc::WaveTable::new(sample_rate)
        }
    }
}


pub struct Engine {
    pub voices: [Voice; 8]
}

impl Engine {
    pub fn new(sample_rate: f64) -> Engine {
        Engine {
            voices: [Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate),
                     Voice::new(sample_rate)]
        }
    }

    pub fn process_sample(&mut self, midi: &MidiController) -> f64 {
        let (a, d, s, r) = midi.adsr;
        let osc_mix = midi.osc_mix;

        // Debug
        let mut signal: f64;
        let note = &midi.notes[0];
        let freq = note.freq;
        let gate = note.gate;

        let mut signal = 0.0;
        for (i, note) in midi.notes.iter().enumerate() {
            let freq = note.freq;
            let gate = note.gate;
            let sound = self.voices[i].osc.step(freq, osc_mix);
        
            self.voices[i].adsr.set_adsr(a, d, s, r);
            signal += self.voices[i].adsr.step(sound, gate);
        }
        
        return signal;
    }
}
