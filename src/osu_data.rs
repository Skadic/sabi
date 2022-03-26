use std::str::FromStr;

use bitflags::bitflags;

#[derive(Clone, Copy, Debug)]
pub enum Countdown {
    NoCountdown,
    Normal,
    Half,
    Double,
}

impl Default for Countdown {
    fn default() -> Self {
        Self::Normal
    }
}

impl TryFrom<u8> for Countdown {
    type Error = &'static str;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use Countdown::*;
        Ok(match v {
            0 => NoCountdown,
            1 => Normal,
            2 => Half,
            3 => Double,
            _ => return Err("Invalid Countdown Id"),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SampleSet {
    NoSample,
    Normal,
    Soft,
    Drum,
}

impl SampleSet {
    pub fn str_rep(&self) -> &'static str {
        use SampleSet::*;
        match self {
            // Todo something is going wrong here. filenames produced include "none-hitnone.wav" which aren't a thing
            //  maybe has to do with this str_rep method?
            NoSample => "normal",
            Normal => "normal",
            Soft => "soft",
            Drum => "drum",
        }
    }
}

impl TryFrom<u8> for SampleSet {
    type Error = &'static str;

    fn try_from(s: u8) -> Result<Self, Self::Error> {
        use SampleSet::*;
        Ok(match s {
            0 => NoSample,
            1 => Normal,
            2 => Soft,
            3 => Drum,
            _ => return Err("Invalid Sample Set ID"),
        })
    }
}

impl FromStr for SampleSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SampleSet::*;
        Ok(match s {
            "None" => NoSample,
            "Normal" => Normal,
            "Soft" => Soft,
            "Drum" => Drum,
            _ => return Err("Invalid Sample Set ID"),
        })
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::Normal
    }
}

bitflags! {
    pub struct Hitsound: u8 {
        const NORMAL = 0b0000_0001;
        const WHISTLE = 0b0000_0010;
        const FINISH = 0b0000_0100;
        const CLAP = 0b0000_1000;
    }
}

impl Hitsound {
    pub fn str_rep(&self) -> Option<&'static str> {
        Some(match *self {
            Self::NORMAL => "normal",
            Self::WHISTLE => "whistle",
            Self::FINISH => "finish",
            Self::CLAP => "clap",
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Standard,
    Taiko,
    CatchTheBeat,
    Mania,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Standard
    }
}

impl TryFrom<u8> for GameMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Standard,
            1 => Self::Taiko,
            2 => Self::CatchTheBeat,
            3 => Self::Mania,
            _ => return Err("Error converting value to GameMode"),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

impl FromStr for OverlayPosition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OverlayPosition::*;
        Ok(match s {
            "NoChange" => NoChange,
            "Below" => Below,
            "Above" => Above,
            _ => return Err("Invalid Overlay Position ID"),
        })
    }
}
