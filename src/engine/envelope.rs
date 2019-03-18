pub struct ADSR {
    a: f64,
    d: f64,
    s: f64,
    r: f64,
    time_per_step: f64,
    last_gate: bool,
    ads_duration: f64,
    r_duration: f64,
    last_attenuate: f64
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
            ads_duration: 0.0,
            r_duration: 0.0,
            last_attenuate: 0.0
        }
    }

    pub fn step(&mut self, gate: bool) -> f64 {
        self.maybe_reset_duration(gate);
        let attenuate: f64;
        if gate {
            self.ads_duration += self.time_per_step;
            if self.ads_duration < self.a {
                attenuate = self.ads_duration / self.a;
            } else {
                let decay_amount: f64;
                if self.d > 0.0 {
                    decay_amount = (self.ads_duration - self.a) / self.d;
                } else {
                    decay_amount = 0.0;
                }
                attenuate = (1.0 - decay_amount).max(self.s);
            }
        } else if (self.last_attenuate > 0.0) & (self.r > 0.0) {
            self.r_duration += self.time_per_step;
            let unbounded_attenuate = 1.0 - self.r_duration / self.r;
            attenuate = unbounded_attenuate.max(0.0).min(self.last_attenuate);
        } else {
            attenuate = 0.0;
        }
        self.last_gate = gate;
        self.last_attenuate = attenuate;
        return attenuate
    }

    pub fn set_adsr(&mut self, a: f64, d: f64, s: f64, r: f64) {
        self.a = a;
        self.d = d;
        self.s = s;
        self.r = r;
    }

    fn maybe_reset_duration(&mut self, gate: bool) {
        if (gate == true) & (self.last_gate == false) {
            self.ads_duration = 0.0;
        } else if (gate == false) & (self.last_gate == true) {
            self.r_duration = 0.0;
        }
    }
}
