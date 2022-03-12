pub mod lance;

pub enum WeaponButton {
    Light,
    Heavy,
    Special,
}

pub trait Weapon {
    fn sheathe(&mut self) -> bool;
    fn reset(&mut self);

    fn light_attack(&mut self) -> Option<crate::AttackIntent>;
    fn heavy_attack(&mut self) -> Option<crate::AttackIntent>;
    fn special_attack(&mut self) -> Option<crate::AttackIntent>;

    fn can_activate(&self, button: WeaponButton) -> bool;
    fn attack_name(&self, button: WeaponButton) -> Option<String>;
}
