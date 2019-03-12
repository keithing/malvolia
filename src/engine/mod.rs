mod osc;
mod envelope;

use data::{SIN_TABLE, SAW_TABLE};

pub struct Engine {
    pub sin_osc: osc::WaveTable,
    pub saw_osc: osc::WaveTable,
    pub adsr: envelope::ADSR
}

impl Engine {
    pub fn new(sample_rate: f64) -> Engine {
        Engine {
            sin_osc: osc::WaveTable::new(sample_rate, SIN_TABLE),
            saw_osc: osc::WaveTable::new(sample_rate, SAW_TABLE),
            adsr: envelope::ADSR::new(sample_rate)
        }
    }
}
