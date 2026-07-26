# R20

R20 is a simplistic D20 roller built in Rust.

## IMPORTANT FOR COMPILING

This project does not include any audio files. If you want sound effects enabled, you must provide your own `.wav` files.

Place the audio files in:

assets/sounds/


The required filenames are:

- `roll.wav` - Played when rolling normally
- `nat20.wav` - Played when rolling a natural 20
- `nat1.wav` - Played when rolling a natural 1

For best compatibility, use `.wav` files.

Make sure the files are named exactly as shown above, otherwise the project may not compile or audio may not work.
