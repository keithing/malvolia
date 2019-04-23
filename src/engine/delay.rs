use std::collections::{VecDeque};

pub struct Delay {
    buffer: VecDeque<f64>,
    offset: usize,
    wet: f64,
    dry: f64
}

impl Delay {
    pub fn new() -> Delay {
        Delay {
            buffer: VecDeque::with_capacity(44100),
            offset: 882, // 20 milliseconds
            wet: 1.0,
            dry: 1.0
        }
    }

    pub fn set_wet(&mut self, wet: f64) {
        self.wet = wet;
    }

    pub fn set_dry(&mut self, dry: f64) {
        self.dry = dry;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn step(&mut self, signal: f64) -> f64 {
        if self.offset == 0 {return signal};
        let mut wet_signal = 0.0;
        if self.buffer.len() > self.offset {
            let b = self.buffer.pop_front();
            wet_signal = match b {
                Some(e) => e,
                _ => 0.0
            };
        }
        self.buffer.push_back(signal);
        return signal * self.dry + wet_signal * self.wet;
    }
}
