use std::{collections::HashMap, fs::DirEntry};

pub fn get_available_replays() -> std::io::Result<Vec<DirEntry>> {
    let files = std::fs::read_dir("res/replays")?
        .map(|entry| entry.expect("unable to get dir entry"))
        .collect();
    Ok(files)
}

pub fn get_available_maps() -> std::io::Result<HashMap<String, DirEntry>> {
    let files = std::fs::read_dir("res/maps")?
        .flat_map(|entry| {
            entry
                .map(|e| std::fs::read_dir(e.path()).unwrap())
                .expect("unable to get dir entry")
        })
        .map(|d| d.unwrap())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                == "osu"
        })
        .map(|entry| {
            (
                format!(
                    "{:x}",
                    md5::compute(std::fs::read_to_string(entry.path()).unwrap())
                ),
                entry,
            )
        })
        .collect::<HashMap<_, _>>();
    Ok(files)
}
