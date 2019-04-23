use engine::osc::SinLFO;

const BUFFER_SIZE: usize = 1200;
const MODULATION_OFFSET: f64 = 400.0;
const NUM_VOICES: usize = 7;
const BASE_OFFSET: f64 = BUFFER_SIZE as f64 - MODULATION_OFFSET - 100.0;
const MODULATION_SPEED_HZ: f64 = 0.25;
const VOICE_RATIO: f64 = 1.0 / NUM_VOICES as f64;

pub struct Chorus {
    lfo: SinLFO,
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
            lfo: SinLFO::new(44100.0),
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
        let modulation_offset = (self.lfo.step(MODULATION_SPEED_HZ) + 1.0) * 0.5 * MODULATION_OFFSET as f64;
        let mut wet_signal = 0.0;
        for i in 0..NUM_VOICES {
            //let voice_offset = (i as f64 - 3.0) * 10.0;
            let voice_offset = 0.0;
            let raw_offset = voice_offset + modulation_offset + BASE_OFFSET;
            let offset0 = raw_offset.floor();
            let offset1 = offset0 + 1.0;
            let tap0 = self.delay_line[wrap_buffer(self.last_step as f64 - offset0)];
            let tap1 = self.delay_line[wrap_buffer(self.last_step as f64 - offset1)];
            wet_signal += (tap0 + (raw_offset - offset0) * (tap1 - tap0)) * VOICE_RATIO;
        }
        self.last_step = wrap_buffer(self.last_step as f64 + 1.0);
        return signal * (1.0 - self.mix) + wet_signal * self.mix;
    }
}
