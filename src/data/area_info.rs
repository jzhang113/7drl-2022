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
    pub map_type: usize,

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

    let prefix_info = AREA_DATA.prefixes[prefix_index];
    let area_info = AREA_DATA.areas[area_index];

    AreaInfo {
        name: get_combined_name(&prefix_info, &area_info),
        map_type: get_combined_generator(&prefix_info, &area_info),
        color_hex: get_combined_color(&prefix_info, &area_info),
    }
}

fn get_combined_name(prefix: &AreaInfo, area: &AreaInfo) -> String {
    [prefix.name.clone(), " ".to_string(), area.name.clone()].concat()
}

fn get_combined_generator(prefix: &AreaInfo, area: &AreaInfo) -> usize {
    if area.map_type != 0 {
        area.map_type
    } else if prefix.map_type != 0 {
        prefix.map_type
    } else {
        0
    }
}

fn get_combined_color(prefix: &AreaInfo, area: &AreaInfo) -> String {
    if prefix.color_hex != default_color() {
        prefix.color_hex
    } else if area.color_hex != default_color() {
        area.color_hex
    } else {
        default_color()
    }
}
