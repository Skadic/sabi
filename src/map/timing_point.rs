use std::str::FromStr;

use bitflags::bitflags;

use crate::osu_data::SampleSet;

#[derive(Debug, Clone, Copy, Default)]
pub struct TimingPoint {
    time: u64,
    beat_length: f64,
    meter: u8,
    sample_set: SampleSet,
    sample_index: u8,
    volume: u8,
    uninherited: bool,
    effects: Effects,
}

impl FromStr for TimingPoint {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split(',');

        macro_rules! quick_parse {
            ($name:ident: $t:ty) => {
                split
                    .next()
                    .ok_or(concat!("Error reading ", stringify!($name)))
                    .and_then(|s| {
                        s.parse::<$t>().map_err(|_| {
                            concat!("Error parsing ", stringify!($name), " to ", stringify!($t))
                        })
                    })?
            };
        }

        let time = quick_parse!(time: u64);
        let beat_length = quick_parse!(beat_length: f64);
        let meter = quick_parse!(meter: u8);
        let sample_set = SampleSet::try_from(quick_parse!(sample_set: u8))?;
        let sample_index = quick_parse!(sample_index: u8);
        let volume = quick_parse!(volume: u8);
        let uninherited = quick_parse!(uninherited: u8) == 1;
        let effects = Effects::from_bits(quick_parse!(effects: u8))
            .ok_or("Error parsing effects bitstring")?;

        Ok(Self {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        })
    }
}

bitflags! {
    pub struct Effects : u8 {
        const KIAI = 0b001;
        const OMIT_BARLINE = 0b100;
    }
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            bits: Default::default(),
        }
    }
}
