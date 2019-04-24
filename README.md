# Malvolia

Malvolia is a proof of concept VST synthesizer programmed
in Rust. It uses [rust-vst](https://github.com/rust-dsp/rust-vst)
for the VST(2) interface and is based on wavetable synthesis.

# Build

To build, first run `make_data.py` (requires numpy).  This
will generate the necessary wave tables as Rust source
code.

# Examples

Here are a few examples of the synth (no extra effects):

- [Jump - Van Halen](https://s3.amazonaws.com/keithingpub/Malvolia/jump.ogg)
- [Kids - Stranger Things](https://s3.amazonaws.com/keithingpub/Malvolia/kids.ogg)
