#[derive(Debug, Default)]
pub struct Metadata {
    title: String,
    title_unicode: String,
    artist: String,
    artist_unicode: String,
    creator: String,
    version: String,
    source: String,
    tags: Vec<String>,
    beatmap_id: u64,
    beatmap_set_id: u64,
}

impl<'a, T> From<T> for Metadata
where
    T: Iterator<Item = &'a str>,
{
    fn from(iter: T) -> Self {
        let mut meta = Self::default();

        for line in iter {
            let mapping = line
                .split(':')
                .map(|split| split.trim())
                .collect::<Vec<_>>();
            let k = mapping[0];
            let v = mapping[1];

            match k {
                "Title" => meta.title = v.to_owned(),
                "TitleUnicode" => meta.title_unicode = v.to_owned(),
                "Artist" => meta.artist = v.to_owned(),
                "ArtistUnicode" => meta.artist_unicode = v.to_owned(),
                "Creator" => meta.creator = v.to_owned(),
                "Version" => meta.version = v.to_owned(),
                "Source" => meta.source = v.to_owned(),
                "Tags" => meta.tags = v.split_whitespace().map(|s| s.to_owned()).collect(),
                "BeatmapID" => meta.beatmap_id = v.parse().unwrap(),
                "BeatmapSetID" => meta.beatmap_set_id = v.parse().unwrap(),
                _ => {}
            }
        }

        meta
    }
}
