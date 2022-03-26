#[derive(Debug, Clone, Copy, Default)]
pub struct Difficulty {
    hp_drain_rate: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tick_rate: f32,
}

impl<'a, T> From<T> for Difficulty
where
    T: Iterator<Item = &'a str>,
{
    fn from(iter: T) -> Self {
        let mut diff = Self::default();

        for line in iter {
            let mapping = line
                .split(':')
                .map(|split| split.trim())
                .collect::<Vec<_>>();
            let k = mapping[0];
            let v = mapping[1];

            match k {
                "HPDrainRate" => diff.hp_drain_rate = v.parse().unwrap(),
                "CircleSize" => diff.circle_size = v.parse().unwrap(),
                "OverallDifficulty" => diff.overall_difficulty = v.parse().unwrap(),
                "ApproachRate" => diff.approach_rate = v.parse().unwrap(),
                "SliderMultiplier" => diff.slider_multiplier = v.parse().unwrap(),
                "SliderTickRate" => diff.slider_tick_rate = v.parse().unwrap(),
                _ => {}
            }
        }

        diff
    }
}
