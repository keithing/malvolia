"""Script to generate data.rs

data.rs contains various wave tables and misc data used by the plugin.
We precompute these values and store them directly in source code to
speed up the plugin.
"""
import os

import numpy as np


NUM_SAMPLES = 2048
TEMPLATE = "pub static {name}: [f64; {size}] = [{data}];"
PRECISION = "%.12f"


def create_sine_table():
    name = "SIN_TABLE"
    radians = np.linspace(0, np.pi * 2, num=NUM_SAMPLES)
    samples = np.sin(radians)
    data = ", ".join([PRECISION % x for x in samples])
    return TEMPLATE.format(name=name, size=len(samples), data=data)

def create_saw_table():
    name = "SAW_TABLE"
    radians = np.linspace(0, np.pi * 2, num=NUM_SAMPLES)
    samples = 1 - radians / np.pi
    data = ", ".join([PRECISION % x for x in samples])
    return TEMPLATE.format(name=name, size=len(samples), data=data)

def create_square_table():
    name = "SQUARE_TABLE"
    radians = np.linspace(0, np.pi * 2, num=NUM_SAMPLES)
    samples = (radians > np.pi)
    data = ", ".join([PRECISION % x for x in samples])
    return TEMPLATE.format(name=name, size=len(samples), data=data)

def create_midi_lookup():
    name = "FREQ_FROM_PITCH"
    a4_pitch = 69
    a4_freq = 440.0
    notes = np.arange(128)
    freqs = np.exp2((notes - a4_pitch) / 12) * a4_freq
    data = ", ".join([PRECISION % x for x in freqs])
    return TEMPLATE.format(name=name, size=len(freqs), data=data)

def create_many_saw_tables():
    amplitude = 1
    freq = 20
    tau = 2 * np.pi
    t = 0
    sample_rate = 44100
    time_step = 1 / sample_rate
    n_samples = 2048

    phase = 0;
    samples = []
    for _ in range(n_samples):
        y = amplitude - (amplitude / np.pi * phase)
        samples.append(y)
        phase = phase + ((tau * freq) / sample_rate)
        if phase > (tau):
            phase = phase - (tau)
    samples = np.array(samples)

    fft_table = np.fft.rfft(samples)
    for _ in range(11):
        n_harmonics = np.int((sample_rate / 2) / freq)
        banded = fft_table.copy()
        banded[n_harmonics:] = 0
        banded[0] = 0
        wave = np.fft.irfft(banded)
        data = ", ".join([PRECISION % x for x in wave])
        name = "SAW_TABLE_{}".format(freq)
        yield TEMPLATE.format(name=name, size=len(samples), data=data)
        freq = freq * 2



def main():
    lines = list(create_many_saw_tables())
    lines.append(create_midi_lookup())
    with open("src/data.rs", "w") as f:
        f.write("\n".join(lines))




if __name__ == "__main__":
    main()
