use data::FREQ_FROM_PITCH;

pub struct MidiController {
    note: u8,
    pub gate: bool,
    pub velocity: f64,
    pub freq: f64,
    pub adsr: (f64, f64, f64, f64),
}


impl MidiController {

    pub fn new() -> MidiController {
        MidiController {
            gate: false,
            note: 69,
            velocity: 100.0,
            freq: 440.0,
            adsr: (0.1, 0.1, 1.0, 0.1),
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

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.note = note;
        self.gate = true;
        self.velocity = velocity as f64;
        self.freq = FREQ_FROM_PITCH[note as usize];
    }

    fn note_off(&mut self, note: u8) {
        if note == self.note {
            self.gate = false;
        }
    }

    fn push_control_change(&mut self, id: u8, level: u8) {
        if id == 0 {self.adsr.0 = (level as f64 / 33.0 - 1.0).exp2();}
        if id == 1 {self.adsr.1 = (level as f64 / 33.0 - 1.0).exp2();}
        if id == 2 {self.adsr.2 = level as f64 / 127.0;}
        if id == 3 {self.adsr.3 = (level as f64 / 33.0 - 1.0).exp2();}
    }
}
