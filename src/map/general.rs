use super::*;

#[derive(Debug)]
pub struct General {
    audio_file: String,
    audio_lead_in: usize,
    preview_time: Option<usize>,
    countdown: Countdown,
    sample_set: SampleSet,
    stack_leniency: f64,
    mode: GameMode,
    letterbox_in_breaks: bool,
    use_skin_sprites: bool,
    always_show_playfield: bool,
    overlay_position: OverlayPosition,
    skin_preference: Option<String>,
    epilepsy_warning: bool,
    countdown_offset: usize,
    special_style: bool,
    widescreen_storyboard: bool,
    samples_match_playback_rate: bool,
}

impl<'a, T> From<T> for General
where
    T: Iterator<Item = &'a str>,
{
    fn from(iter: T) -> Self {
        let mut general = Self::default();

        for line in iter {
            let mapping = line
                .split(':')
                .map(|split| split.trim())
                .collect::<Vec<_>>();
            let k = mapping[0];
            let v = mapping[1];

            match k {
                "AudioFilename" => general.audio_file = v.into(),
                "AudioLeadIn" => general.audio_lead_in = v.parse().unwrap(),
                "PreviewTime" => {
                    general.preview_time = if v == "-1" { None } else { v.parse().ok() }
                }
                "Countdown" => {
                    general.countdown = Countdown::try_from(v.parse::<u8>().unwrap()).unwrap()
                }
                "SampleSet" => general.sample_set = SampleSet::from_str(v).unwrap(),
                "StackLeniency" => general.stack_leniency = v.parse().unwrap(),
                "Mode" => general.mode = GameMode::try_from(v.parse::<u8>().unwrap()).unwrap(),
                "LetterboxInBreaks" => general.letterbox_in_breaks = v.parse::<u8>().unwrap() != 0,
                "UseSkinSprites" => general.use_skin_sprites = v.parse::<u8>().unwrap() != 0,
                "OverlayPosition" => {
                    general.overlay_position = OverlayPosition::from_str(v).unwrap()
                }
                "SkinPreference" => general.skin_preference = Some(v.to_owned()),
                "EpilepsyWarning" => general.epilepsy_warning = v.parse::<u8>().unwrap() != 0,
                "CountdownOffset" => general.countdown_offset = v.parse().unwrap(),
                "SpecialStyle" => general.special_style = v.parse::<u8>().unwrap() != 0,
                "WidescreenStoryboard" => {
                    general.widescreen_storyboard = v.parse::<u8>().unwrap() != 0
                }
                "SamplesMatchPlaybackRate" => {
                    general.samples_match_playback_rate = v.parse::<u8>().unwrap() != 0
                }
                _ => {}
            }
        }

        general
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            audio_file: Default::default(),
            audio_lead_in: 0,
            preview_time: None,
            countdown: Countdown::Normal,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: GameMode::Standard,
            letterbox_in_breaks: false,
            use_skin_sprites: false,
            always_show_playfield: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: None,
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}
