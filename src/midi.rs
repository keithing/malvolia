use data::FREQ_FROM_PITCH;

pub struct Note {
    pub pitch: u8,
    pub freq: f64,
    pub velocity: f64,
    pub gate: bool
}

impl Note {
    fn new() -> Note {
        Note {
            pitch: 69,
            freq: 440.0,
            velocity: 100.0,
            gate: false
        }
    }
}


pub struct MidiController {
    pub notes: [Note; 8],
    pub adsr: (f64, f64, f64, f64),
    pub osc_mix: [f64; 3]
}


impl MidiController {

    pub fn new() -> MidiController {
        MidiController {
            notes: [Note::new(),
                    Note::new(),
                    Note::new(),
                    Note::new(),
                    Note::new(),
                    Note::new(),
                    Note::new(),
                    Note::new()],
            adsr: (0.01, 0.0, 1.0, 0.1),
            osc_mix: [1.0, 0.0, 0.0]
        }
    }

    /// Process an incoming midi event.
    ///
    /// The midi data is split up like so:
    ///
    /// `data[0]`: Contains the status and the channel. Source: [source]
    /// `data[1]`: Contains the supplemental data for the message - so, if this was a NoteOn then
    ///            this would contain the note.
    /// `data[2]`: Further supplemental data. Would be velocity in the case of a NoteOn message.
    ///
    /// [source]: http://www.midimountain.com/midi/midi_status.htm
    pub fn process_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            176 => self.push_control_change(data[1], data[2]),
            _ => (),
        }
    }

    fn note_on(&mut self, pitch: u8, velocity: u8) {
        for note in self.notes.iter_mut() {
            if !note.gate {
                note.pitch = pitch;
                note.freq = FREQ_FROM_PITCH[pitch as usize];
                note.velocity = velocity as f64;
                note.gate = true;
                break;
            }
        }
    }

    fn note_off(&mut self, pitch: u8) {
        for i in 0..self.notes.len() {
            if pitch == self.notes[i].pitch {
                self.notes[i].gate = false;
            }
        }
    }

    fn push_control_change(&mut self, id: u8, level: u8) {
        if id == 0 {self.adsr.0 = (level as f64 / 33.0).exp2() - 1.0;}
        if id == 1 {self.adsr.1 = (level as f64 / 33.0).exp2() - 1.0;}
        if id == 2 {self.adsr.2 = level as f64 / 127.0;}
        if id == 3 {self.adsr.3 = (level as f64 / 33.0).exp2() - 1.0;}
        if id == 4 {self.set_osc_mix(level)}
    }

    fn set_osc_mix(&mut self, level: u8) {
        let reference = level as f64 / 127.0;
        let n_points = self.osc_mix.len();
        let spacing = 1.0 / (n_points - 1) as f64;
        let mut point = 0.0;
        for i in 0..n_points {
            let dist = ((reference - point) / spacing).abs();
            if dist >= 1.0 {
                self.osc_mix[i] = 0.0;
            } else {
                self.osc_mix[i] = 1.0 - dist;
            }
            point += spacing;
        }
    }
}
