rltk::embedded_resource!(AREA_DATA, "../../data/area_info.yaml");

mod area_info;
pub use area_info::{get_random_area, AreaInfo};
