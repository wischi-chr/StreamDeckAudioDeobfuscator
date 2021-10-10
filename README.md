# *.streamDeckAudio Deobfuscator
The streamDeckAudio Deobfuscator (sdad) is a tool that coverts `*.wav` files to `*.streamDeckAudio` and vice versa. This is mainly just a very tiny project for me to get used to [Rust](https://www.rust-lang.org/). So if you know Rust and see something awful, please open an issue I'd love to lean more.

## Legal stuff
Just because you can convert `*.streamDeckAudio` files to `*.wav` files with this tool, does not mean that you are allowed to use the wave files. This tool is not related to Elgato.

## Technical stuff
The audio files `*.streamDeckAudio` are internally just [RIFF WAVE (*.wav)](https://en.wikipedia.org/wiki/WAV) files, but transformed with a single byte XOR with the value `0x5E`.
This tool just takes a single argument (either a `*.wav` or a `*.streamDeckAudio`) and transforms it into the other type by XOR-ing the entire file.

**Usage:** `sdad.exe <input-file>`
