pub struct ADSR {
    a: f64,
    d: f64,
    s: f64,
    r: f64,
    time_per_step: f64,
    last_gate: bool,
    last_signal: f64,
    duration: f64,
    release_duration: f64,
    ads_stage: bool,
    r_stage: bool
}

impl ADSR {
    pub fn new(sample_rate: f64) -> ADSR {
        ADSR {
            time_per_step: 1.0/sample_rate,
            a: 0.7,
            d: 0.5,
            s: 0.5,
            r: 1.0,
            last_gate: false,
            last_signal: 0.0,
            duration: 0.0,
            release_duration: 0.0,
            ads_stage: false,
            r_stage: false
        }
    }

    pub fn set_adsr(&mut self, a: f64, d: f64, s: f64, r: f64) {
        self.a = a;
        self.d = d;
        self.s = s;
        self.r = r;
    }

    pub fn step(&mut self, signal: f64, gate: bool) -> f64 {
        let mut out_signal = signal;
        self.set_adsr_stage(gate);
        if self.ads_stage {
            self.duration += self.time_per_step;
            if self.duration < self.a {
                out_signal = out_signal * (self.duration / self.a);
            } else {
                let attenuate = 1.0 - (self.duration - self.a) / self.d;
                if attenuate > self.s {
                    out_signal = out_signal * attenuate;
                } else {
                    out_signal = out_signal * self.s; 
                }
            }
        } else if self.r_stage {
            if self.last_signal.abs() > 0.0 {
                self.release_duration += self.time_per_step;
                let attenuate = 1.0 - self.release_duration / self.r;
                if attenuate > 0.0 {
                    out_signal = self.s * self.last_signal * attenuate;
                } else {
                    out_signal = 0.0;
                }
            } else {
                out_signal = 0.0;
            } 
        } else {
            out_signal = 0.0;
        }
        self.last_gate = gate;
        self.last_signal = signal;
        return out_signal
    }

    fn set_adsr_stage(&mut self, gate: bool) {
        if (gate == true) & (self.last_gate == false) {
            self.ads_stage = true;
            self.r_stage = false;
            self.duration = 0.0;
        } else if (gate == false) & (self.last_gate == true) {
            self.ads_stage = false;
            self.r_stage = true;
            self.release_duration = 0.0;
        }
    }
}
