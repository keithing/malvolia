use engine::osc::SinLFO;

const BUFFER_SIZE: usize = 1200;
const MODULATION_OFFSET: f64 = 100.0;
const NUM_VOICES: usize = 3;
const BASE_OFFSET: f64 = BUFFER_SIZE as f64 - MODULATION_OFFSET - 100.0;
const MODULATION_SPEED_HZ: f64 = 0.25;
const VOICE_RATIO: f64 = 1.0 / NUM_VOICES as f64;

pub struct Chorus {
    lfo: [SinLFO; NUM_VOICES],
    mix: f64,
    delay_line: [f64; BUFFER_SIZE],
    last_step: usize
}

fn wrap_buffer(i: f64) -> usize {
    let buf_size_float = BUFFER_SIZE as f64;
    if i < 0.0 {return (i + buf_size_float) as usize};
    if i >= buf_size_float {return (i - buf_size_float) as usize};
    return i as usize;
}

impl Chorus {
    pub fn new() -> Chorus {
        Chorus {
            lfo: [SinLFO::new(44100.0), SinLFO::new(44100.0), SinLFO::new(44100.0)],
            mix: 0.0,
            delay_line: [0.0; BUFFER_SIZE],
            last_step: 0
        }
    }

    pub fn set_mix(&mut self, mix: f64) {
        self.mix = mix;
    }

    pub fn step(&mut self, signal: f64) -> f64 {
        self.delay_line[self.last_step] = signal;
        if self.mix <= 0.01 {return signal};
        let mut wet_signal = 0.0;
        for i in 0..NUM_VOICES {
            let lfo_val = self.lfo[i].step(MODULATION_SPEED_HZ * (i + 1) as f64);
            let modulation_offset =  lfo_val * MODULATION_OFFSET as f64;
            let raw_offset = modulation_offset + BASE_OFFSET;
            let offset0 = raw_offset.floor();
            let offset1 = offset0 + 1.0;
            let tap0 = self.delay_line[wrap_buffer(self.last_step as f64 - offset0)];
            let tap1 = self.delay_line[wrap_buffer(self.last_step as f64 - offset1)];
            wet_signal += tap0 + (raw_offset - offset0) * (tap1 - tap0);
        }
        self.last_step = wrap_buffer(self.last_step as f64 + 1.0);
        return signal + wet_signal * self.mix;
    }
}
