use data::{SIN_TABLE, SQUARE_TABLE, SAW_TABLE};

pub struct WaveTable {
    pub size: f64,
    pub last_step: f64,
    pub ratio: f64,
    tables: [[f64; 2048]; 3]
}

impl WaveTable {
    pub fn new(sample_rate: f64) -> WaveTable {
        let size = (SIN_TABLE.len()) as f64;
        WaveTable {
            size: size,
            last_step: 0.0,
            ratio: size / sample_rate,
            tables: [SIN_TABLE, SAW_TABLE, SQUARE_TABLE]
        }
    }

    pub fn step(&mut self, freq: f64, mix: [f64; 3]) -> f64 {
        let step_size = self.ratio * freq;
        let raw_step = step_size + self.last_step;
        let next_step = if raw_step > self.size {
            raw_step - self.size
        } else {
            raw_step
        };

        let i = next_step.floor() as usize;
        let mut signal = 0.0;
        for (j, table) in self.tables.iter().enumerate() {
            signal += table[i] * mix[j];
        }
        self.last_step = next_step;
        return signal
    }
}
