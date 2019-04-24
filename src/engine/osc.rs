use data;


fn get_wavetable_octave(freq: f64) -> usize {
    if freq >= 11024.0 {return 0};
    if freq >= 5511.0 {return 1};
    if freq >= 2755.0 {return 2};
    if freq >= 1377.0 {return 3};
    if freq >= 688.0  {return 4};
    if freq >= 343.0 {return 5};
    if freq >= 171.0 {return 6};
    if freq >= 85.0 {return 7};
    if freq >= 42.0 {return 8};
    if freq >= 20.0 {return 9};
    return 10;
}

pub struct WaveTable {
    pub size: f64,
    pub last_step: f64,
    pub ratio: f64
}

impl WaveTable {
    pub fn new(sample_rate: f64) -> WaveTable {
        let size = 1024.0;
        WaveTable {
            size: size,
            last_step: 0.0,
            ratio: size / sample_rate
        }
    }

    pub fn step(&mut self, freq: f64, mix: [f64; 2]) -> f64 {
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


        let octave = get_wavetable_octave(freq);
        let mut signal = 0.0;

        let table = &data::SAW_TABLE[octave];
        let table2 = &data::SQUARE_TABLE[octave];
        let saw0 = table[i as usize];
        let saw1 = table[j as usize];
        let square0 = table2[i as usize];
        let square1 = table2[j as usize];
        signal += (saw0 + rem * (saw1 - saw0)) * mix[0];
        signal += (square0 + rem * (square1 - square0)) * mix[1];
        self.last_step = next_step;
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
