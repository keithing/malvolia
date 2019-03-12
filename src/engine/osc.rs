pub struct WaveTable {
    pub size: f64,
    pub last_step: f64,
    pub last_freq: f64,
    pub ratio: f64,
    table: [f64; 2048]
}

impl WaveTable {
    pub fn new(sample_rate: f64, table: [f64; 2048]) -> WaveTable {
        let size = (table.len()) as f64;
        WaveTable {
            size: size,
            last_step: 0.0,
            last_freq: 440.0,
            ratio: size / sample_rate,
            table: table
        }
    }

    pub fn step(&mut self, freq: Option<f64>) -> f64 {
        let current_freq = match freq {
            Some(hz) => hz,
            None => self.last_freq
        };
        let step_size = self.ratio * current_freq;
        let raw_step = step_size + self.last_step;
        let next_step = if raw_step > self.size {
            raw_step - self.size
        } else {
            raw_step
        };
        let floor = next_step.floor();
        let signal = self.table[floor as usize];
        self.last_freq = current_freq;
        self.last_step = next_step;
        return signal
    }
}
