use crate::map::color_data::Color;
use crate::map::general::General;
use crate::map::hit_object::HitObject;
use crate::osu_data::*;
use bitflags::bitflags;
use std::collections::HashMap;
use std::str::FromStr;

use self::color_data::ColorData;
use self::difficulty::Difficulty;
use self::metadata::Metadata;
use self::timing_point::TimingPoint;

pub mod color_data;
pub mod difficulty;
pub mod general;
pub mod hit_object;
pub mod metadata;
pub mod timing_point;

#[derive(Default, Debug)]
pub struct Beatmap {
    general: General,
    metadata: Metadata,
    difficulty: Difficulty,
    timing_points: Vec<TimingPoint>,
    color_data: ColorData,
    hit_objects: Vec<HitObject>,
}

impl Beatmap {
    /// Get a reference to the beatmap's hit objects.
    pub fn hit_objects(&self) -> &[HitObject] {
        self.hit_objects.as_ref()
    }

    /// Get a reference to the beatmap's general.
    pub fn general(&self) -> &General {
        &self.general
    }

    /// Get a reference to the beatmap's metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Get a reference to the beatmap's difficulty.
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    /// Get a reference to the beatmap's timing points.
    pub fn timing_points(&self) -> &[TimingPoint] {
        self.timing_points.as_ref()
    }

    /// Get a reference to the beatmap's color data.
    pub fn color_data(&self) -> &ColorData {
        &self.color_data
    }
}

impl FromStr for Beatmap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! get_section {
            ($sec_name:ident) => {
                s.lines()
                    .skip_while(|&line| {
                        line.trim() != concat!("[", stringify!($sec_name), "]").to_owned()
                    })
                    .skip(1)
                    .take_while(|&line| !line.trim().is_empty() || line.trim().starts_with("["))
                    .filter(|&line| !line.is_empty())
            };
        }

        let general = General::from(get_section!(General));
        let metadata = Metadata::from(get_section!(Metadata));
        let difficulty = Difficulty::from(get_section!(Difficulty));

        let timing_points = get_section!(TimingPoints)
            .map(|line| TimingPoint::from_str(line.trim()).unwrap())
            .collect();

        let hit_objects = get_section!(HitObjects)
            .map(|line| HitObject::from_str(line.trim()).unwrap())
            .collect();

        let color_mappings = get_section!(Colours)
            .map(|line| {
                let mut split = line.split(':').map(|token| token.trim());
                (
                    split.next().unwrap().to_owned(),
                    split
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|color_val| color_val.parse::<u8>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(key, values)| (key, Color::from(&values[..])))
            .collect::<HashMap<String, Color>>();

        Ok(Self {
            general,
            metadata,
            difficulty,
            timing_points,
            color_data: ColorData::from(color_mappings),
            hit_objects,
        })
    }
}
