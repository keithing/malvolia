pub struct Filter {
    cutoff: f64,
    buf0: f64,
    buf1: f64,
    buf2: f64,
    buf3: f64,
    resonance: f64
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            cutoff: 0.1,
            buf0: 0.0,
            buf1: 0.0,
            buf2: 0.0,
            buf3: 0.0,
            resonance: 0.2
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f64) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f64) {
        self.resonance = resonance;
    }

    pub fn process(&mut self, signal: f64) -> f64 {
        let feedback = self.resonance + self.resonance/(1.0 - self.cutoff);
        let bandpass = self.buf0 - self.buf1;
        self.buf0 += self.cutoff * (signal - self.buf0 + feedback * bandpass);
        self.buf1 += self.cutoff * (self.buf0 - self.buf1);
        self.buf2 += self.cutoff * (self.buf1 - self.buf2);
        self.buf3 += self.cutoff * (self.buf2 - self.buf3);
        return self.buf3;
    }
}
