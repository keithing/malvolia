use data;

pub struct WaveTable {
    pub size: f64,
    pub last_step: f64,
    pub ratio: f64
}

impl WaveTable {
    pub fn new(sample_rate: f64) -> WaveTable {
        let size = 2048.0;
        WaveTable {
            size: size,
            last_step: 0.0,
            ratio: size / sample_rate
        }
    }

    pub fn step(&mut self, freq: f64, mix: [f64; 3]) -> f64 {
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


        let k: usize;
        if freq <= 30.0 {
            k = 0;
        } else if freq <= 60.0 {
            k = 1;
        } else if freq <= 120.0 {
            k = 2;
        } else if freq <= 240.0 {
            k = 3;
        } else if freq <= 480.0 {
            k = 4;
        } else if freq <= 960.0 {
            k = 5;
        } else if freq <= 1920.0 {
            k = 6;
        } else if freq <= 3840.0 {
            k = 7;
        } else if freq <= 7680.0 {
            k = 8;
        } else if freq <= 15360.0 {
            k = 9;
        } else {
            k = 10;
        }
        let tables = [&data::SAW_TABLE_20,
                      &data::SAW_TABLE_40,
                      &data::SAW_TABLE_80,
                      &data::SAW_TABLE_160,
                      &data::SAW_TABLE_320,
                      &data::SAW_TABLE_640,
                      &data::SAW_TABLE_1280,
                      &data::SAW_TABLE_2560,
                      &data::SAW_TABLE_5120,
                      &data::SAW_TABLE_10240,
                      &data::SAW_TABLE_20480];
        let mut signal = 0.0;
        let x0 = tables[k][i as usize];
        let x1 = tables[k][j as usize];
        signal += (x0 + (x1 - x0) * rem);
        self.last_step = next_step;
        return signal
    }
}
