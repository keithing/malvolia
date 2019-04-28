use data;

// wavtables are optimized to be a single array which combines tables
// for each octave.  For example, we have a table of size 4 for the
// highest octave, size 8 for the next highest, etc.  The length of
// the table is therefore the sum of these octave sizes plus padding.
// The first 4 elements are zero padded so that the table of size 4
// is offset as the 4th index, the 256 size table has an offset of
// 256, etc.
const OCTAVE_SIZES: [usize; 9] = [4, 8, 16, 32, 64, 128, 256, 512, 1024];

pub struct WaveTable {
    last_freq: f64,
    last_step: f64,
    freq_cutoffs: Vec<f64>,
    step_ratios: Vec<f64>
}

fn add_and_wrap(x: f64, y: f64, size: usize) -> f64 {
    let z = x + y;
    if z >= (size as f64) {return z - (size as f64)};
    return z;
}

impl WaveTable {
    pub fn new(sample_rate: f64) -> WaveTable {
        let mut freq_cutoffs: Vec<f64> = Vec::new();
        let mut step_ratios: Vec<f64> = Vec::new();
        for size in OCTAVE_SIZES.iter() {
            freq_cutoffs.push(sample_rate / *size as f64);
            step_ratios.push(*size as f64 / sample_rate);
        }
        WaveTable {
            last_freq: 0.0,
            last_step: 0.0,
            freq_cutoffs: freq_cutoffs,
            step_ratios: step_ratios
        }
    }

    fn get_size_and_step(&self, freq: f64) -> (usize, f64) {
        for (i, cutoff) in self.freq_cutoffs.iter().enumerate() {
            if freq > *cutoff {
                return (OCTAVE_SIZES[i], self.step_ratios[i] * freq);
            }
        }
        let i = self.freq_cutoffs.len() - 1;
        return (OCTAVE_SIZES[i], self.step_ratios[i] * freq);
    }

    pub fn step(&mut self, freq: f64, mix: [f64; 2]) -> f64 {
        if freq != self.last_freq {
            self.last_freq = freq;
            self.last_step = 0.0;
        }
        let (size, step) = self.get_size_and_step(freq);
        let next = add_and_wrap(self.last_step, step, size);
        let next_floor = next.floor();
        let next_ceil = add_and_wrap(next_floor, 1.0, size);
        let rem = next - next_floor;


        // size is designed to be same as offset
        let mut signal = 0.0;
        let table = &data::SAW_TABLE;
        let x = table[size + next_floor as usize];
        let y = table[size + next_ceil as usize];
        signal += x + rem * (y - x);
        self.last_step = next;
        return signal
    }
}


#[derive(Copy, Clone)]
pub struct SinLFO {
    pub size: f64,
    pub last_step: f64,
    pub ratio: f64
}

impl SinLFO {
    pub fn new(sample_rate: f64) -> SinLFO {
        let size = 44100.0;
        SinLFO {
            size: size,
            last_step: 0.0,
            ratio: size / sample_rate
        }
    }

    pub fn step(&mut self, freq: f64) -> f64 {
        let step_size = self.ratio * freq;
        let raw_step = step_size + self.last_step;
        let next_step = if raw_step >= self.size {
            raw_step - self.size
        } else {
            raw_step
        };
        let i = next_step.floor();
        let j = if (i + 1.0) >= self.size {
            i + 1.0 - self.size
        } else {
            i + 1.0
        };
        let rem = next_step - i;

        let mut signal = 0.0;

        let table = &data::LFO_SIN_TABLE;
        let sin0 = table[i as usize];
        let sin1 = table[j as usize];
        signal += sin0 + rem * (sin1 - sin0);
        self.last_step = next_step;
        return signal;
    }
}
