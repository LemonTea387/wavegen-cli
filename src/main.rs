use std::{
    fs::File,
    io::{BufWriter, Write},
};

use args::SineWaves;
use clap::Parser;
use wavegen::{sine, Waveform};

mod args;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let mut waveform = Waveform::<f32>::new(args.sampling_rate);
    let mut total_amplitude = 0.;

    let SineWaves(waves) = args.signals;
    waves.into_iter().for_each(|wave| {
        println!("{}", wave);
        total_amplitude += wave.amplitude.0;
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
    let samples = waveform
        .iter()
        .take(points)
        .map(|v| v / total_amplitude)
        .map(|value: f32| -> sin_gen::DataType {
            match args.data_type {
                args::DataType::Float => {
                    sin_gen::DataType::FloatData(<sin_gen::FloatData>::from(value))
                }
                args::DataType::Hex16 => {
                    sin_gen::DataType::HexData16(<sin_gen::HexData16>::from(value))
                }
                args::DataType::Hex32 => {
                    sin_gen::DataType::HexData32(<sin_gen::HexData32>::from(value))
                }
            }
        });

    let file = File::create(args.out_file)?;
    let mut writer = BufWriter::new(file);

    match args.save_mode {
        args::SaveMode::Wave => todo!(),
        args::SaveMode::Raw => {
            samples.for_each(|val| { writer.write_all(val.to_string().as_bytes()) }.unwrap());
        }
    }
    Ok(())
}
