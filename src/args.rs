use std::{fmt::Display, path::PathBuf, str::FromStr};

use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Degree(pub f32);
impl Default for Degree {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Clone, Debug)]
pub struct Amplitude(pub f32);
impl Default for Amplitude {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Clone, Default, Debug)]
pub struct SineWave {
    pub frequency: f32,
    pub phase: Degree,
    pub amplitude: Amplitude,
}

impl FromStr for SineWave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let piece_pattern = Regex::new(r"[fpa]((\d+\.?\d*)|(\.\d+)|(\d+))").unwrap();
        let res =
            piece_pattern
                .find_iter(s)
                .try_fold(SineWave::default(), |mut signal, element| {
                    if element.is_empty() {
                        return Err("Empty piece".to_string());
                    }
                    let elem_str = element.as_str();
                    match elem_str.chars().nth(0) {
                        Some('f') => {
                            let val = elem_str
                                .chars()
                                .skip(1)
                                .collect::<String>()
                                .parse::<f32>()
                                .map_err(|_| "Parsing float error")?;
                            signal.frequency = val;
                        }
                        Some('p') => {
                            let val = elem_str
                                .chars()
                                .skip(1)
                                .collect::<String>()
                                .parse::<f32>()
                                .map_err(|_| "Parsing float error")?;
                            signal.phase = Degree(val);
                        }
                        Some('a') => {
                            let val = elem_str
                                .chars()
                                .skip(1)
                                .collect::<String>()
                                .parse::<f32>()
                                .map_err(|_| "Parsing float error")?;
                            signal.amplitude = Amplitude(val);
                        }
                        Some(e) => return Err(format!("Invalid token {}", e)),
                        None => return Ok(signal),
                    };
                    Ok(signal)
                });
        res
    }
}

impl Display for SineWave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Sine Wave
Frequency   : {}
Phase       : {}
Amplitude   : {}
",
            self.frequency, self.phase.0, self.amplitude.0,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SineWaves(pub Vec<SineWave>);

impl FromStr for SineWaves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(SineWave::from_str)
                .collect::<Result<Vec<SineWave>, Self::Err>>()?,
        ))
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SaveMode {
    Wave,
    Raw,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DataForm {
    Real,
    IQ,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DataType {
    Float,
    Hex16,
    Hex32
}

#[derive(Parser)]
pub struct Args {
    /// Sampling rate for the waveform
    #[arg(short = 'f', long = "fsample")]
    pub sampling_rate: f32,
    /// Comma separated sine waves each with format: [fW][pX][aY], where WXY are f32 for frequency, phase(degrees), amplitude. At least one parameter must exist.
    /// Default values:
    ///     f:0.0
    ///     p:0.0
    ///     a:1.0
    #[arg(short, long, verbatim_doc_comment)]
    pub signals: SineWaves,

    /// How long to sample the waveform for? (s)
    /// Mutually exclusive with sample_points (-p --points).
    #[arg(short = 't', long = "time", group = "constraint")]
    pub sampling_time: Option<f32>,
    /// How many points to sample the waveform for?
    /// Mutually exclusive with sampling_time (-t --time).
    #[arg(short = 'p', long = "points", group = "constraint")]
    pub sample_points: Option<usize>,

    /// Output file
    #[arg(short = 'o', long = "out", default_value = "out.txt")]
    pub out_file: PathBuf,

    // NOTE: Not needed yet
    // /// Data form mode, real or iq (hilbert transform) data.
    // #[arg(long = "data_form", default_value = "real")]
    // pub data_form: DataForm,

    /// Data type to output
    #[arg(short = 'd', long = "data_type", default_value = "float")]
    pub data_type: DataType,

    /// Save mode
    /// Raw for txt file of <real> <imaginary> pair.
    /// Wave to save to wave file.
    #[arg(value_enum, verbatim_doc_comment)]
    pub save_mode: SaveMode,
}
