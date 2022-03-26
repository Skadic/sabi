use std::{collections::HashMap, fs::DirEntry, io::BufRead, str::FromStr};

use replay::Replay;

use crate::{
    interpolation::*,
    map::Beatmap,
};

mod file_reading;
#[allow(unused)]
mod map;
mod osu_data;
#[allow(unused)]
mod replay;

mod interpolation;

fn main() -> std::io::Result<()> {
    let replays = file_reading::get_available_replays()?;
    let maps = file_reading::get_available_maps()?;
    let (_map, _replay) = choose_replay(replays.as_slice(), &maps).unwrap();

    Ok(())
}

fn choose_replay(
    replays: &[DirEntry],
    maps: &HashMap<String, DirEntry>,
) -> Result<(Beatmap, Replay), &'static str> {
    println!("Choose Replay:");
    replays
        .iter()
        .map(|entry| entry.file_name())
        .enumerate()
        .for_each(|(i, s)| println!("{}: {}", i, s.to_string_lossy()));

    let choice = {
        let mut c = Err(());
        while let Err(_) = c {
            c = std::io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .map_err(|_| ());
            if matches!(c, Err(_)) || c.as_ref().unwrap() >= &replays.len() {
                eprintln!("Please input a number between 0 and {}", replays.len() - 1);
                c = Err(())
            }
        }
        c.unwrap()
    };

    let replay_file = &replays[choice];
    let replay_bytes = std::fs::read(replay_file.path()).unwrap();
    let replay = Replay::try_from(&replay_bytes[..]).unwrap();

    let map_file = match maps.get(&replay.map_md5_hash) {
        Some(file) => file,
        None => {
            eprintln!(
                "Map for this replay is unavailable (MD5 Hash: {})",
                replay.map_md5_hash
            );
            return Err("Map unavailable");
        }
    };
    println!("{:?}", replay.map_md5_hash);
    println!("{:?}", map_file.path());

    let map_str = std::fs::read_to_string(map_file.path()).unwrap();
    let map = Beatmap::from_str(&map_str[..]).unwrap();

    for obj in map.hit_objects() {
        //println!("{:?}", obj);
    }

    println!(
        "linear: {:?}",
        interpolate_linear::<usize, f64>((0, 0), (100, 100), 0.656)
    );

    /*/for i in 0..=10usize {
        let lambda = i as f64 * 0.1;
        println!(
            "circle: {:?}",
            
        );
    }*/
    let ((x, y), rad) = find_circle_rnd::<i8, f64>((-3, 0), (0, 3), (3, 0));
    let start = (3f64, 0f64);
    let end = (-3f64, 0f64);

    println!("atan: {:?}", interpolate_perfect_circle(start, end, (x, y), rad, 0.5));

    Ok((map, replay))
}
