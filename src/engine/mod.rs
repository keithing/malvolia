mod osc;
mod envelope;
mod filter;

use midi::MidiController;

pub struct Voice {
    pub adsr: envelope::ADSR,
    pub filter: filter::Filter,
    pub osc: osc::WaveTable
}

impl Voice {
    pub fn new(sample_rate: f64) -> Voice {
        Voice {
            adsr: envelope::ADSR::new(sample_rate),
            filter: filter::Filter::new(),
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
                     Voice::new(sample_rate)],
        }
    }

    pub fn process_sample(&mut self, midi: &MidiController) -> f64 {
        let (a, d, s, r) = midi.adsr;
        let cutoff = midi.cutoff;
        let resonance = midi.resonance;
        let osc_mix = midi.osc_mix;
        let mut output = 0.0;
        for (i, note) in midi.notes.iter().enumerate() {
            let freq = note.freq;
            let gate = note.gate;
            self.voices[i].adsr.set_adsr(a, d, s, r);
            self.voices[i].filter.set_cutoff(cutoff);
            self.voices[i].filter.set_resonance(resonance);

            let mut signal = self.voices[i].osc.step(freq, osc_mix);
            signal = self.voices[i].filter.process(signal);
            output += signal * self.voices[i].adsr.step(gate);
        }

        return output
    }
}
