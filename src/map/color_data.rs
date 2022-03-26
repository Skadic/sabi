use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ColorData {
    combo_colors: Vec<Color>,
    slider_track: Option<Color>,
    slider_border: Option<Color>,
}

impl From<HashMap<String, Color>> for ColorData {
    fn from(map: HashMap<String, Color>) -> Self {
        let mut i = 1;
        let mut data = ColorData::default();
        while map.contains_key(&format!("Color{}", i)) {
            data.combo_colors.push(map[&format!("Color{}", i)]);
            i += 1;
        }

        data.slider_track = map.get("SliderTrackOverride").copied();
        data.slider_border = map.get("SliderBorder").copied();

        data
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<&[u8]> for Color {
    fn from(slice: &[u8]) -> Self {
        Self {
            r: slice[0],
            g: slice[1],
            b: slice[2],
        }
    }
}
