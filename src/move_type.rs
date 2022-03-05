use crate::{AttackIntent, RangeType};
use rltk::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AttackType {
    Sweep,
    Punch,
    Stun,
    Push,
    Dodge,
    Recover,
    // enemy specific attacks
    Haymaker,
    Ranged,
}

#[derive(PartialEq, Copy, Clone)]
pub enum AttackTiming {
    Slow,
    Fast,
}

#[derive(PartialEq, Copy, Clone)]
pub enum AttackTrait {
    Damage { amount: i32 },
    Knockback { amount: i32 },
    Movement,
    Equipment,
    Heal { amount: i32 },
}

// check if an attack is can be executed
// this returns the tile that will hit the target
pub fn is_attack_valid(
    attack_type: &AttackType,
    from_point: Point,
    target: Point,
) -> Option<Point> {
    let range_type = get_attack_range(attack_type);
    let shape = get_attack_shape(attack_type);

    for tile in crate::range_type::resolve_range_at(&range_type, from_point) {
        let affected_tiles = crate::range_type::resolve_range_at(&shape, tile);

        if affected_tiles.contains(&target) {
            return Some(tile);
        }
    }

    None
}

// return all points that are affected by an attack
pub fn each_attack_target(attack_type: &AttackType, from_point: Point) -> Vec<Point> {
    let shape = get_attack_shape(attack_type);
    crate::range_type::resolve_range_at(&shape, from_point)
}

// convert an attack into an intent that can be executed by the event system
pub fn get_attack_intent(
    attack_type: &AttackType,
    loc: Point,
    attack_modifier: Option<AttackType>,
) -> AttackIntent {
    AttackIntent {
        main: *attack_type,
        modifier: attack_modifier,
        loc,
    }
}

pub fn get_attack_range(attack_type: &AttackType) -> RangeType {
    match attack_type {
        AttackType::Sweep => RangeType::Single,
        AttackType::Punch => RangeType::Square { size: 1 },
        AttackType::Stun => RangeType::Square { size: 1 },
        AttackType::Push => RangeType::Square { size: 1 },
        AttackType::Dodge => RangeType::Square { size: 2 },
        AttackType::Recover => RangeType::Single,
        AttackType::Haymaker => RangeType::Square { size: 1 },
        AttackType::Ranged => RangeType::Square { size: 3 },
    }
}

pub fn get_attack_power(attack_type: &AttackType) -> i32 {
    match attack_type {
        AttackType::Sweep => 1,
        AttackType::Punch => 1,
        AttackType::Stun => 0,
        AttackType::Push => 0,
        AttackType::Dodge => 0,
        AttackType::Recover => 0,
        AttackType::Haymaker => 3,
        AttackType::Ranged => 1,
    }
}

pub fn get_attack_shape(attack_type: &AttackType) -> RangeType {
    match attack_type {
        AttackType::Sweep => RangeType::Square { size: 1 },
        AttackType::Punch => RangeType::Single,
        AttackType::Stun => RangeType::Single,
        AttackType::Push => RangeType::Single,
        AttackType::Dodge => RangeType::Single,
        AttackType::Recover => RangeType::Single,
        AttackType::Haymaker => RangeType::Single,
        AttackType::Ranged => RangeType::Single,
    }
}

pub fn get_attack_speed(attack_type: &AttackType) -> i32 {
    match attack_type {
        AttackType::Sweep => 0,
        AttackType::Punch => 1,
        AttackType::Stun => 2,
        AttackType::Push => 0,
        AttackType::Dodge => 2,
        AttackType::Recover => 0,
        AttackType::Haymaker => -4,
        AttackType::Ranged => 0,
    }
}

pub fn get_attack_guard(attack_type: &AttackType) -> i32 {
    match attack_type {
        AttackType::Sweep => 0,
        AttackType::Punch => 0,
        AttackType::Stun => 0,
        AttackType::Push => 0,
        AttackType::Dodge => -2,
        AttackType::Recover => 0,
        AttackType::Haymaker => 2,
        AttackType::Ranged => -4,
    }
}

pub fn get_attack_name(attack_type: &AttackType) -> String {
    let name = match attack_type {
        AttackType::Sweep => "sweep",
        AttackType::Punch => "punch",
        AttackType::Stun => "stun",
        AttackType::Push => "push",
        AttackType::Dodge => "dodge",
        AttackType::Recover => "recover",
        AttackType::Haymaker => "haymaker",
        AttackType::Ranged => "shoot",
    };

    name.to_string()
}

pub fn get_attack_traits(attack_type: &AttackType) -> Vec<AttackTrait> {
    match attack_type {
        AttackType::Sweep => vec![AttackTrait::Damage { amount: 1 }],
        AttackType::Punch => vec![AttackTrait::Damage { amount: 1 }],
        AttackType::Stun => vec![AttackTrait::Damage { amount: 0 }],
        AttackType::Push => vec![AttackTrait::Knockback { amount: 2 }],
        AttackType::Dodge => vec![AttackTrait::Movement],
        AttackType::Recover => vec![AttackTrait::Heal { amount: 2 }],
        AttackType::Haymaker => vec![AttackTrait::Damage { amount: 2 }],
        AttackType::Ranged => vec![AttackTrait::Damage { amount: 1 }],
    }
}
