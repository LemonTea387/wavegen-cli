use std::{fs::File, io::Write, path::PathBuf};

use args::SineWaves;
use clap::Parser;
use wavegen::{sine, Waveform};

mod args;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let mut waveform = Waveform::<f32>::new(args.sampling_rate);

    let SineWaves(waves) = args.signals;
    waves.into_iter().for_each(|wave| {
        println!("{}", wave);
        waveform.add_component(sine!(
            wave.frequency,
            wave.amplitude.0,
            wave.phase.0.to_radians()
        ))
    });
    let points: usize;
    if let Some(point) = args.sample_points {
        points = point;
    } else {
        points = (args.sampling_rate * args.sampling_time.unwrap()) as usize;
    }
    let samples = waveform.iter().take(points);

    let out = match args.out_file {
        Some(path) => path,
        None => PathBuf::from("out.txt"),
    };
    let mut file = File::create(out)?;

    match args.save_mode {
        args::SaveMode::Wave => todo!(),
        args::SaveMode::Raw => {
            samples.for_each(|val| file.write_fmt(format_args!("{} {}\n", val, 0)).unwrap());
        }
    }
    Ok(())
}
