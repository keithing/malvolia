"""Script to generate data.rs

data.rs contains various wave tables and misc data used by the plugin.
We precompute these values and store them directly in source code to
speed up the plugin.
"""
import os

import numpy as np
from scipy.signal import decimate

NUM_SAMPLES = 1024
PRECISION = "%.6f"

def sample_sine_wave(n_samples=NUM_SAMPLES):
    oversample_rate = 10
    radians = np.linspace(0, np.pi * 2, num=n_samples * oversample_rate)
    samples = np.sin(radians)
    return decimate(samples, oversample_rate)

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

def create_midi_lookup():
    name = "FREQ_FROM_PITCH"
    a4_pitch = 69
    a4_freq = 440.0
    notes = np.arange(128)
    freqs = np.exp2((notes - a4_pitch) / 12) * a4_freq
    data = ", ".join([PRECISION % x for x in freqs])
    template = "pub static {name}: [f64; {size}] = [{data}];"
    return template.format(name=name, size=len(freqs), data=data)

def create_wavetables(samples):
    """Prevent aliasing by applying a band pass per octave."""
    n_octaves = int(np.log2(len(samples)) + 1)
    fft_table = np.fft.rfft(samples)
    wavetable = []
    for octave in range(n_octaves):
        n_harmonics = 2 ** octave
        fft_bandpass = fft_table.copy()
        fft_bandpass[0] = 0
        fft_bandpass[(n_harmonics + 1):] = 0
        waveform_bandpass = np.fft.irfft(fft_bandpass)
        wavetable.append(waveform_bandpass)
    return wavetable


def serialize_wavetable(wavetable, name="wavetable"):
    n, m = len(wavetable), len(wavetable[0])
    wavetable_strs = []
    for waveform in wavetable:
        waveform_str = ", ".join([PRECISION % x for x in waveform])
        wavetable_strs.append("[" + waveform_str + "]")
    array = "[" + ",".join(wavetable_strs) + "]"
    template = "pub static {name}: [[f64; {m}]; {n}] = {array};"
    return template.format(name=name, m=m, n=n, array=array)

def main():
    saw_table = create_wavetables(sample_saw_wave())
    square_table = create_wavetables(sample_square_wave())

    lines = []
    lines.append(serialize_wavetable(saw_table, "SAW_TABLE"))
    lines.append(serialize_wavetable(square_table, "SQUARE_TABLE"))
    lines.append(create_midi_lookup())
    with open("src/data.rs", "w") as f:
        f.write("\n".join(lines))

if __name__ == "__main__":
    main()
