use serde::Deserialize;

lazy_static! {
    static ref AREA_DATA: AreaData = load_area_data();
}

#[derive(Deserialize, Debug)]
struct AreaData {
    prefixes: Vec<AreaInfo>,
    areas: Vec<AreaInfo>,
}

#[derive(Deserialize, Debug)]
pub struct AreaInfo {
    pub name: String,

    #[serde(default)]
    pub map_gen_type: usize,

    #[serde(default = "default_color")]
    pub color_hex: String,
}

fn default_color() -> String {
    "#FFFFFF".to_string()
}

fn load_area_data() -> AreaData {
    rltk::link_resource!(AREA_DATA, "../../data/area_info.yaml");

    // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
    let raw_data = rltk::embedding::EMBED
        .lock()
        .get_resource("../../data/area_info.yaml".to_string())
        .unwrap();
    let raw_string =
        std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");

    serde_yaml::from_str(&raw_string).expect("Unable to parse file")
}

pub fn get_random_area(rng: &mut rltk::RandomNumberGenerator) -> AreaInfo {
    let prefix_index = rng.range(0, AREA_DATA.prefixes.len());
    let area_index = rng.range(0, AREA_DATA.areas.len());

    AreaInfo {
        name: [
            AREA_DATA.prefixes[prefix_index].name.clone(),
            " ".to_string(),
            AREA_DATA.areas[area_index].name.clone(),
        ]
        .concat(),
        map_gen_type: 0,
        color_hex: default_color(),
    }
}
