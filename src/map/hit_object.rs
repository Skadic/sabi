use std::str::FromStr;

use crate::{
    map::hit_object,
    osu_data::{Hitsound, SampleSet},
};

use bitflags::bitflags;

bitflags! {
    struct HitObjectMeta : u8 {
        const HIT_CIRCLE = 0b0000_0001;
        const SLIDER = 0b0000_0010;
        const NEW_COMBO = 0b0000_0100;
        const SPINNER = 0b0000_1000;
        const COLOR_SKIP_COUNT_0 = 0b0001_0000;
        const COLOR_SKIP_COUNT_1 = 0b0010_0000;
        const COLOR_SKIP_COUNT_2 = 0b0100_0000;
        const OSU_MANIA_HOLD = 0b1000_0000;
    }
}

impl HitObjectMeta {
    fn combo_skip_count(&self) -> u8 {
        (self.bits() >> 4) & 0b0000_0111
    }
}

#[derive(Default, Debug, Clone)]
pub struct CustomHitSample {
    hit_sample_data: HitSampleData,
    index: u8,
    volume: u8,
    file_name: String,
}

impl FromStr for CustomHitSample {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = vec![0u8; 4];
        for (i, n) in s.split(':').take(4).enumerate() {
            vals[i] = n
                .parse()
                .map_err(|_| "Error parsing value for Custom Hit Sample")?;
        }
        let normal_set = SampleSet::try_from(vals[0])?;
        let addition_set = SampleSet::try_from(vals[1])?;
        let hit_sample_data = HitSampleData {
            normal_set,
            addition_set,
        };
        let index = vals[2];
        let file_name = format!(
            "{}-hit{}{}.wav",
            normal_set.str_rep(),
            addition_set.str_rep(),
            if index > 1 {
                index.to_string()
            } else {
                "".to_owned()
            }
        );

        Ok(Self {
            hit_sample_data,
            index,
            volume: vals[3],
            file_name,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct HitSampleData {
    normal_set: SampleSet,
    addition_set: SampleSet,
}

impl FromStr for HitSampleData {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = vec![0u8; 2];
        for (i, n) in s.split(':').take(2).enumerate() {
            vals[i] = n
                .parse()
                .map_err(|_| "Error parsing value for Custom Hit Sample")?;
        }
        let normal_set = SampleSet::try_from(vals[0])?;
        let addition_set = SampleSet::try_from(vals[1])?;

        Ok(Self {
            normal_set,
            addition_set,
        })
    }
}

#[derive(Debug)]
pub enum HitObjectData {
    Circle,
    Slider(SliderData),
    Spinner(u64), // Parameter is duration of the spinner
}

#[derive(Debug)]
pub struct HitObject {
    x: u16,
    y: u16,
    timestamp: u64,
    hit_object_meta: HitObjectMeta,
    hit_sound: Hitsound,
    hit_sample: CustomHitSample,
    object_data: HitObjectData,
}

impl HitObject {
    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn hit_sound(&self) -> Hitsound {
        self.hit_sound
    }

    pub fn hit_sample(&self) -> &CustomHitSample {
        &self.hit_sample
    }

    pub fn object_data(&self) -> &HitObjectData {
        &self.object_data
    }
}

impl FromStr for HitObject {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HitObjectData::*;

        let mut tokens = s.split(',');
        let x = tokens
            .next()
            .ok_or("Error reading x while parsing hit object")?
            .parse()
            .map_err(|_| "Error parsing x position token as integer")?;

        let y = tokens
            .next()
            .ok_or("Error reading y while parsing hit object")?
            .parse()
            .map_err(|_| "Error parsing y position token as integer")?;

        let timestamp = tokens
            .next()
            .ok_or("Error reading timestamp while parsing hit object")?
            .parse()
            .map_err(|_| "Error parsing timestamp token as integer")?;

        let hit_object_meta = HitObjectMeta::from_bits(
            tokens
                .next()
                .ok_or("Error reading hit object metadata while parsing hit object")?
                .parse()
                .map_err(|_| "Error parsing hit object metadata token as integer")?,
        )
        .ok_or("Error parsing bits as Hit Object Meta")?;

        let hit_sound = Hitsound::from_bits(
            tokens
                .next()
                .ok_or("Error reading hitsound data while parsing hit object")?
                .parse()
                .map_err(|_| "Error parsing hitsound data token as integer")?,
        )
        .ok_or("Error parsing bits as Hitsound")?;

        // Now come the object params (possibly). Since these are optional and the hit sample value comes at the end,
        // we need to collect all remaining tokens to see if there are actually object parameters.
        let mut hit_object_data = tokens.collect::<Vec<_>>();

        let hit_sample = if let Some(_) = hit_object_data.last() {
            if hit_object_data.last().unwrap().contains(":") {
                CustomHitSample::from_str(hit_object_data.pop().unwrap())?
            } else {
                CustomHitSample::default()
            }
        } else {
            CustomHitSample::default()
        };

        let object_data;
        if hit_object_data.is_empty() {
            object_data = Circle;
        } else if let Ok(spinner_duration) = hit_object_data.first().unwrap().parse() {
            // If the first object data value is a number, it is a spinner with its duration
            object_data = Spinner(spinner_duration);
        } else {
            // Otherwise it's a slider
            object_data = Slider(SliderData::try_from(&hit_object_data[..])?);
        }

        Ok(Self {
            x,
            y,
            timestamp,
            hit_object_meta,
            hit_sound,
            hit_sample,
            object_data,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SliderCurveType {
    Bezier,
    CentripetalCatmullRom,
    Linear,
    PerfectCircle,
}

impl FromStr for SliderCurveType {
    type Err = &'static str;

    fn from_str(c: &str) -> Result<Self, Self::Err> {
        use SliderCurveType::*;
        Ok(match c {
            "B" => Bezier,
            "C" => CentripetalCatmullRom,
            "L" => Linear,
            "P" => PerfectCircle,
            _ => return Err("Invalid Slider Curve Type"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SliderData {
    curve_type: SliderCurveType,
    curve_points: Vec<(i16, i16)>,
    slides: usize,
    length: f64,
    edge_sounds: Vec<u8>,
    edge_sets: Vec<HitSampleData>,
}

impl<'a> TryFrom<&'a [&'a str]> for SliderData {
    type Error = &'static str;

    fn try_from(tokens: &'a [&'a str]) -> Result<Self, Self::Error> {
        let mut slider_data = tokens[0].split('|');
        let curve_type = SliderCurveType::from_str(
            slider_data
                .next()
                .ok_or("No token to read for curve type")?,
        )?;
        let curve_points = slider_data
            .map(|pair| {
                let mut split = pair.split(':');
                let x = split
                    .next()
                    .expect("No token for x position while parsing slider curve point")
                    .parse()
                    .expect("Error parsing x position for slider curve point");
                let y = split
                    .next()
                    .expect("No token for y position while parsing slider curve point")
                    .parse()
                    .expect("Error parsing y position for slider curve point");
                (x, y)
            })
            .collect();

        let slides = tokens[1].parse().map_err(|_| "Error parsing slide count")?;
        let length = tokens[2].parse().map_err(|_| "Error parsing slide count")?;
        let edge_sounds = tokens
            .get(3)
            .unwrap_or(&"")
            .split('|')
            .filter(|s| !s.is_empty())
            .map(|t| t.parse().expect("Error parsing sound id"))
            .collect();

        // TODO these might have to be string inputs?
        let edge_sets = tokens
            .get(4)
            .unwrap_or(&"")
            .split('|')
            .filter(|s| !s.is_empty())
            .map(|sample_str| {
                HitSampleData::from_str(sample_str).expect("unable to parse hit sample data")
            })
            .collect();

        Ok(SliderData {
            curve_type,
            curve_points,
            slides,
            length,
            edge_sounds,
            edge_sets,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpinnerData {
    end_time: u64,
}
