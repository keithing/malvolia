"""Script to generate data.rs

data.rs contains various wave tables and misc data used by the plugin.
We precompute these values and store them directly in source code to
speed up the plugin.
"""
import os

import numpy as np


NUM_SAMPLES = 2048
TEMPLATE = "pub static {name}: [f64; {size}] = [{data}];"


def create_sine_table():
    name = "SIN_TABLE"
    radians = np.linspace(0, np.pi * 2, num=NUM_SAMPLES)
    samples = np.sin(radians)
    data = ", ".join(["%.12f" % x for x in samples])
    return TEMPLATE.format(name=name, size=len(samples), data=data)

def create_saw_table():
    name = "SAW_TABLE"
    radians = np.linspace(0, np.pi * 2, num=NUM_SAMPLES)
    samples = 1 - (1 / np.pi) * radians
    data = ", ".join(["%.12f" % x for x in samples])
    return TEMPLATE.format(name=name, size=len(samples), data=data)

def create_midi_lookup():
    name = "FREQ_FROM_PITCH"
    a4_pitch = 69
    a4_freq = 440.0
    notes = np.arange(128)
    freqs = np.exp2((notes - a4_pitch) / 12) * a4_freq
    data = ", ".join(["%.12f" % x for x in freqs])
    return TEMPLATE.format(name=name, size=len(freqs), data=data)


def main():
    lines = [create_sine_table(), create_saw_table(), create_midi_lookup()]
    with open("src/data.rs", "w") as f:
        f.write("\n".join(lines))


if __name__ == "__main__":
    main()
