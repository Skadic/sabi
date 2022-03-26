use std::{collections::HashMap, io::BufReader};

use bitflags::bitflags;

use crate::osu_data::GameMode;

bitflags! {
    pub struct Mods : u32 {
        const NONE	= 0;
        const NO_FAIL = 1;
        const EASY = 1 << 1;
        const TOUCH_DEVICE = 1 << 2;
        const HIDDEN = 1 << 3;
        const HARD_ROCK = 1 << 4;
        const SUDDEN_DEATH = 1 << 5;
        const DOUBLE_TIME = 1 << 6;
        const RELAX = 1 << 7;
        const HALF_TIME = 1 << 8;
        const NIGHTCORE	= 1 << 9;
        const FLASHLIGHT = 1 << 10;
        const AUTOPLAY = 1 << 11;
        const SPUN_OUT = 1 << 12;
        const RELAX2 = 1 << 13;
        const PERFECT = 1 << 14;
        const KEY4 = 1 << 15;
        const KEY5 = 1 << 16;
        const KEY6 = 1 << 17;
        const KEY7 = 1 << 18;
        const KEY8 = 1 << 19;
        const KEY_MOD = Self::KEY4.bits | Self::KEY5.bits | Self::KEY6.bits | Self::KEY7.bits | Self::KEY8.bits;
        const FADE_IN = 1 << 20;
        const RANDOM = 1 << 21;
        const LAST_MOD = 1 << 22;
        const TARGET_PRACTICE = 1 << 23;
        const KEY9 = 1 << 24;
        const COOP = 1 << 25;
        const KEY1 = 1 << 26;
        const KEY3 = 1 << 27;
        const KEY2 = 1 << 28;
        const SCORE_V2 = 1 << 29;
        const MIRROR = 1 << 30;
    }
}

bitflags! {
    struct InputKeys: u32 {
        const M1 = 1;
        const M2 = 1 << 1;
        const K1 = 1 << 2;
        const K2 = 1 << 3;
        const SMOKE = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReplayFrame {
    time_delta: u64,
    x: f32,                // x coord between 0 - 512
    y: f32,                // y coord between 0 - 384
    input_keys: InputKeys, // bitwise combination of keys/mouse pressed (M1 = 1, M2 = 2, K1 = 4, K2 = 8, Smoke = 16)
}

impl Default for ReplayFrame {
    fn default() -> Self {
        Self {
            time_delta: Default::default(),
            x: Default::default(),
            y: Default::default(),
            input_keys: InputKeys::from_bits(0u32).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Replay {
    pub mode: GameMode,
    pub game_ver: u32,
    pub map_md5_hash: String,
    pub player_name: String,
    pub replay_md5_hash: String,
    pub n_300: u16,
    pub n_100: u16,
    pub n_50: u16,
    pub n_geki: u16,
    pub n_katu: u16,
    pub n_miss: u16,
    pub total_score: u32,
    pub max_combo: u16,
    pub perfect_combo: bool, // represented as 1 byte in the file
    pub mods: Mods,          // represented as a 32 bit int
    pub life_bar_graph: HashMap<usize, f64>,
    pub time_stamp: u64,
    pub compressed_data_length: u32, // in bytes
    pub replay_data: Vec<ReplayFrame>,
    pub online_score_id: u64,
    pub total_hit_accuracy: f64, // only for target practice mod
}

impl Replay {
    pub fn replay_frames(&self) -> &[ReplayFrame] {
        &self.replay_data
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ReplayFrame> {
        self.replay_data.iter()
    }
}

impl<'a> TryFrom<&'a [u8]> for Replay {
    type Error = &'a str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes = value.into_iter().cloned();

        macro_rules! read_byte {
            () => {
                bytes.next().ok_or("Error reading byte")?
            };
        }

        macro_rules! read_short {
            () => {{
                let err = "Error reading short";
                let lo = bytes.next().ok_or(err)?;
                let hi = bytes.next().ok_or(err)?;
                u16::from_le_bytes([lo, hi])
            }};
        }

        macro_rules! read_int {
            () => {{
                let err = "Error reading int";
                let l1 = bytes.next().ok_or(err)?;
                let l2 = bytes.next().ok_or(err)?;
                let l3 = bytes.next().ok_or(err)?;
                let l4 = bytes.next().ok_or(err)?;
                u32::from_le_bytes([l1, l2, l3, l4])
            }};
            // For use in lambdas, in which case we can't use '?'
            (panic) => {{
                let err = "Error reading int";
                let l1 = bytes.next().expect(err);
                let l2 = bytes.next().expect(err);
                let l3 = bytes.next().expect(err);
                let l4 = bytes.next().expect(err);
                u32::from_le_bytes([l1, l2, l3, l4])
            }};
        }

        macro_rules! read_long {
            () => {{
                let err = "Error reading long";
                let l1 = bytes.next().ok_or(err)?;
                let l2 = bytes.next().ok_or(err)?;
                let l3 = bytes.next().ok_or(err)?;
                let l4 = bytes.next().ok_or(err)?;
                let l5 = bytes.next().ok_or(err)?;
                let l6 = bytes.next().ok_or(err)?;
                let l7 = bytes.next().ok_or(err)?;
                let l8 = bytes.next().ok_or(err)?;
                u64::from_le_bytes([l1, l2, l3, l4, l5, l6, l7, l8])
            }};
            // For use in lambdas, in which case we can't use '?'
            (panic) => {{
                let err = "Error reading int";
                let l1 = bytes.next().expect(err);
                let l2 = bytes.next().expect(err);
                let l3 = bytes.next().expect(err);
                let l4 = bytes.next().expect(err);
                let l5 = bytes.next().expect(err);
                let l6 = bytes.next().expect(err);
                let l7 = bytes.next().expect(err);
                let l8 = bytes.next().expect(err);
                u64::from_le_bytes([l1, l2, l3, l4, l5, l6, l7, l8])
            }};
        }

        macro_rules! read_uleb128 {
            () => {{
                let err = "Error reading uleb128";
                let mut acc = 0u64;
                let mut i = 0;
                let mut current = bytes.next().ok_or(err)?;
                // read lower significant bytes if exist
                while current & 0b1000_0000 > 0 {
                    acc += ((current & 0b0111_1111) as u64) << (i * 7);
                    i += 1;
                    current = bytes.next().ok_or(err)?;
                }
                // Read most significant byte
                acc += (current as u64) << (i * 7);

                acc
            }};
        }

        macro_rules! read_string {
            () => {{
                let is_present = read_byte!() == 0x0b;
                if is_present {
                    let byte_len = read_uleb128!() as usize;
                    let mut byte_vec = vec![0u8; byte_len];
                    for i in 0..byte_len {
                        byte_vec[i] = bytes.next().ok_or("Error reading byte for String")?;
                    }
                    String::from_utf8(byte_vec)
                        .map_err(|_| "Invalid UTF-8 in replay file String value")?
                } else {
                    String::new()
                }
            }};
        }

        let mode = GameMode::try_from(read_byte!())?;
        let game_ver = read_int!();
        let map_md5_hash = read_string!();
        let player_name = read_string!();
        let replay_md5_hash = read_string!();
        let n_300 = read_short!();
        let n_100 = read_short!();
        let n_50 = read_short!();
        let n_geki = read_short!();
        let n_katu = read_short!();
        let n_miss = read_short!();
        let total_score = read_int!();
        let max_combo = read_short!();
        let perfect_combo = read_byte!() == 1;
        let mods = Mods::from_bits(read_int!()).ok_or("Error reading mods ")?;
        let life_bar_graph = read_string!()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|pair| {
                let mut split = pair.trim().split('|');
                (
                    split.next().map(|v| v.parse().unwrap()).unwrap(),
                    split.next().map(|v| v.parse().unwrap()).unwrap(),
                )
            })
            .collect::<HashMap<usize, f64>>();
        let time_stamp = read_long!();
        let compressed_data_length = read_int!() as u32;
        let replay_data = {
            let mut compressed_replay_data = vec![0u8; compressed_data_length as usize];
            for i in 0..compressed_data_length as usize {
                compressed_replay_data[i] =
                    bytes.next().ok_or("Error reading compressed replay data")?;
            }

            let mut comp_reader = BufReader::new(&compressed_replay_data[..]);
            let mut decompressed_replay_data = vec![];
            lzma_rs::lzma_decompress(&mut comp_reader, &mut decompressed_replay_data)
                .map_err(|_| "Error decompressing replay data")?;
            let decompressed_replay_data = String::from_utf8(decompressed_replay_data).unwrap();
            let mut data_iter = decompressed_replay_data.split(',');

            let mut frames = vec![];

            while let Some(data) = data_iter.next() {
                let mut iter = data.split("|");
                let time_delta = iter.next().map(|s| s.parse::<i64>().unwrap()).unwrap();
                let x = iter
                    .next()
                    .ok_or("Error reading x")
                    .and_then(|s| s.parse().map_err(|_| "Error parsing f32"))?;
                let y = iter
                    .next()
                    .ok_or("Error reading y")
                    .and_then(|s| s.parse().map_err(|_| "Error parsing f32"))?;

                // Don't fully unwrap input keys yet. If we're at the special frame, this value will be the seed instead and therefore not a valid input_keys bitstring
                let input_keys = InputKeys::from_bits(
                    iter.next()
                        .ok_or("Error reading y")
                        .and_then(|s| s.parse().map_err(|_| "Error parsing u32"))?,
                );
                // Special Frame has this werid value as time delta
                if time_delta == -12345 {
                    // TODO implement Seed. Idk what it's for tho
                    break;
                }
                let input_keys = input_keys.ok_or("Invalid input keys value")?;

                frames.push(ReplayFrame {
                    time_delta: unsafe { std::mem::transmute(time_delta) },
                    x,
                    y,
                    input_keys,
                });
            }
            frames
        };
        let online_score_id = read_long!();
        let total_hit_accuracy = if mods.contains(Mods::TARGET_PRACTICE) {
            f64::from_bits(read_long!())
        } else {
            0f64
        };

        Ok(Self {
            mode,
            game_ver,
            map_md5_hash,
            player_name,
            replay_md5_hash,
            n_300,
            n_100,
            n_50,
            n_geki,
            n_katu,
            n_miss,
            total_score,
            max_combo,
            perfect_combo,
            mods,
            life_bar_graph,
            time_stamp,
            compressed_data_length,
            replay_data,
            online_score_id,
            total_hit_accuracy,
        })
    }
}
