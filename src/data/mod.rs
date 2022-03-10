rltk::embedded_resource!(AREA_NAME, "../../data/area_name.yaml");

mod area_info;
pub use area_info::{get_random_area, AreaInfo};
