use data::FREQ_FROM_PITCH;


pub fn midi_note_to_freq(midi_note: &u8) -> f64 {
    return FREQ_FROM_PITCH[*midi_note as usize];
}
