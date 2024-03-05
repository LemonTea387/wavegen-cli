# Composite Sine wave generator
A very simple composite sine wave generator that takes in an arbitrary amount of sine waves and composites them together into a composite sine wave and writes it to file.
This is a small cli for quick generation of sine waveforms to be used for manual testing.

## Building
### Cargo
```
cargo b -r
```

## Usage
```
sin-gen -f 44100 -s f123a400,f440.0a1000,f349.228a500.0 -t2.0 raw
```
