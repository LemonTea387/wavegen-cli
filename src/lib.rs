use std::fmt::Display;

pub struct FloatData(f32);
pub struct HexData16(u16);
pub struct HexData32(u32);

pub enum DataType {
    FloatData(FloatData),
    HexData16(HexData16),
    HexData32(HexData32),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::FloatData(FloatData(x)) => write!(f,"{} {}\n", x, 0),
            DataType::HexData16(HexData16(x)) => write!(f,"{} {}\n", x, 0),
            DataType::HexData32(HexData32(x)) => write!(f,"{} {}\n", x, 0),
        }
    }
}

impl From<f32> for FloatData {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<f32> for HexData16 {
    fn from(value: f32) -> Self {
        let quantised_stepsize = f32::powi(2.0, -15);
        if value > 0. {
            // This maps the range from 0->1 : 0 -> 32767 (0x7FFF)
            let mut low_quantised = (value / quantised_stepsize) as u16;
            // However, the range is only to 32767(0x7FFF)
            // Considered lost to quantization
            if low_quantised == 0x8000 {
                low_quantised -= 1;
            }
            Self(low_quantised)
        } else {
            // For negative, -1 -> 0 : 32768(0x8000) -> 65535(0xFFFF)
            let quantised = ((2. + value) / quantised_stepsize) as u16;
            Self(quantised)
        }
    }
}

impl From<f32> for HexData32 {
    fn from(value: f32) -> Self {
        let quantised_stepsize = f32::powi(2.0, -31);
        if value > 0. {
            // This maps the range from 0->1 : 0 -> 32767 (0x7FFF)
            let mut low_quantised = (value / quantised_stepsize) as u32;
            // However, the range is only to 32767(0x7FFF)
            // Considered lost to quantization
            if low_quantised == 0x80000000 {
                low_quantised -= 1;
            }
            Self(low_quantised)
        } else {
            // For negative, -1 -> 0 : 32768(0x8000) -> 65535(0xFFFF)
            let quantised = ((2. + value) / quantised_stepsize) as u32;
            Self(quantised)
        }
    }
}
