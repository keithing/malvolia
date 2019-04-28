"""Script to generate data.rs

data.rs contains various wave tables and misc data used by the plugin.
We precompute these values and store them directly in source code to
speed up the plugin.
"""
import os

import numpy as np
from scipy.signal import decimate, resample

NUM_SAMPLES = 2048
PRECISION = "%.6f"

def sample_saw_wave(n_samples=NUM_SAMPLES):
    oversample_rate = 10
    radians = np.linspace(0, np.pi * 2, num=n_samples * oversample_rate)
    samples = 1 - radians / np.pi
    return decimate(samples, oversample_rate)

def sample_square_wave(n_samples=NUM_SAMPLES):
    oversample_rate = 10
    radians = np.linspace(0, np.pi * 2, num=n_samples * oversample_rate)
    samples = (radians > np.pi)
    return decimate(samples, oversample_rate)

def create_lfo_table(n_samples=NUM_SAMPLES):
    radians = np.linspace(0, np.pi * 2, num=n_samples)
    samples = np.sin(radians)
    samples = (samples + 1) / 2  # scale to 0/1 range
    array = "[" + ", ".join([PRECISION % x for x in samples]) + "]"
    template = "pub static {name}: [f64; {n}] = {array};"
    return template.format(name="LFO_SIN_TABLE", n=n_samples, array=array)

def create_midi_lookup():
    name = "FREQ_FROM_PITCH"
    a4_pitch = 69
    a4_freq = 440.0
    notes = np.arange(128)
    freqs = np.exp2((notes - a4_pitch) / 12) * a4_freq
    data = ", ".join([PRECISION % x for x in freqs])
    template = "pub static {name}: [f64; {size}] = [{data}];"
    return template.format(name=name, size=len(freqs), data=data)

def create_wavetables(samples, name):
    """Prevent aliasing by applying a band pass per octave."""
    # buffer the first 4 values so that the size of wavetable is
    # equal to its offset.  For example, if we have the octave 4
    # we only need a wavetable of size 2 ** 4 = 16.  We want wavetable[16]
    # to wavetable[16 + 16] to represent the cycle for this octave.
    wavetable = [0.0, 0.0, 0.0, 0.0]
    for octave in range(2, 11):
        waveform_bandpass = resample(samples, 2 ** octave)
        wavetable.extend(waveform_bandpass)
    waveform_strs = ", ".join([PRECISION % x for x in wavetable])
    template = "pub static {name}: [f64; {n}] = [{array}];"
    return template.format(name=name, n=len(wavetable), array=waveform_strs)

def main():
    saw_table = create_wavetables(sample_saw_wave(), "SAW_TABLE")
    square_table = create_wavetables(sample_square_wave(), "SQUARE_TABLE")
    lines = []
    lines.append(saw_table)
    lines.append(square_table)
    lines.append(create_lfo_table(44100))
    lines.append(create_midi_lookup())
    with open("src/data.rs", "w") as f:
        f.write("\n".join(lines))

if __name__ == "__main__":
    main()
