use super::{Weapon, WeaponButton};
use crate::AttackIntent;

pub struct Lance {
    state: LanceState,
    level: u8,
}

#[derive(PartialEq)]
enum LanceState {
    Sheathed,
    Ready,
    Attack { prev_count: u8 },
    Running,
    Counter,
    Guard,
    Wait,
}

enum LanceAttack {
    DrawAttack,
    Thrust { level: u8 },
    Charge,
    Sweep,
}

fn get_attack_name(attack: LanceAttack) -> String {
    match attack {
        LanceAttack::DrawAttack => "Draw Atk",
        LanceAttack::Thrust { level } => match level {
            1 => "Thrust I",
            2 => "Thrust II",
            3 => "Thrust III",
            4 => "Charge Thrust",
            _ => unreachable!(),
        },
        LanceAttack::Charge => "Charge",
        LanceAttack::Sweep => "Sweep",
    }
    .to_string()
}

impl Lance {
    pub fn new() -> Self {
        Self {
            state: LanceState::Sheathed,
            level: 0,
        }
    }

    fn next_state(&self, button: WeaponButton) -> Option<(LanceAttack, LanceState)> {
        match button {
            WeaponButton::Light => self.next_light_state(),
            WeaponButton::Heavy => self.next_heavy_state(),
            WeaponButton::Special => self.next_special_state(),
        }
    }

    fn next_light_state(&self) -> Option<(LanceAttack, LanceState)> {
        match self.state {
            // draw attack
            LanceState::Sheathed => Some((
                LanceAttack::DrawAttack,
                LanceState::Attack { prev_count: 1 },
            )),
            // mid thrust 1
            LanceState::Ready => Some((
                LanceAttack::Thrust { level: 1 },
                LanceState::Attack { prev_count: 1 },
            )),
            // mid thrust 2 / 3
            LanceState::Attack { prev_count } => {
                if prev_count < 3 {
                    Some((
                        LanceAttack::Thrust {
                            level: prev_count + 1,
                        },
                        LanceState::Attack {
                            prev_count: prev_count + 1,
                        },
                    ))
                } else {
                    None
                }
            }
            // final thrust
            LanceState::Running => Some((
                LanceAttack::Thrust { level: 4 },
                LanceState::Attack { prev_count: 1 },
            )),
            LanceState::Counter => None,
            // guard thrust
            LanceState::Guard => Some((
                LanceAttack::Thrust { level: 1 },
                LanceState::Attack { prev_count: 1 },
            )),
            _ => None,
        }
    }

    fn next_heavy_state(&self) -> Option<(LanceAttack, LanceState)> {
        match self.state {
            // sweep
            LanceState::Ready => Some((LanceAttack::Sweep, LanceState::Wait)),
            LanceState::Attack { prev_count } => {
                if prev_count < 3 {
                    Some((LanceAttack::Sweep, LanceState::Wait))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn next_special_state(&self) -> Option<(LanceAttack, LanceState)> {
        match self.state {
            LanceState::Ready => Some((LanceAttack::Charge, LanceState::Running)),
            _ => None,
        }
    }
}

impl Weapon for Lance {
    fn sheathe(&mut self) -> bool {
        if self.state == LanceState::Sheathed {
            return false;
        }

        self.state = LanceState::Sheathed;
        return true;
    }

    fn reset(&mut self) {
        if self.state != LanceState::Sheathed {
            self.state = LanceState::Ready;
        }
    }

    fn light_attack(&mut self) -> Option<AttackIntent> {
        if let Some((attack, next_state)) = self.next_state(WeaponButton::Light) {
            self.state = next_state;
        }
        None
    }

    fn heavy_attack(&mut self) -> Option<AttackIntent> {
        if let Some((attack, next_state)) = self.next_state(WeaponButton::Heavy) {
            self.state = next_state;
        }
        None
    }

    fn special_attack(&mut self) -> Option<AttackIntent> {
        if let Some((attack, next_state)) = self.next_state(WeaponButton::Special) {
            self.state = next_state;
        }
        None
    }

    fn can_activate(&self, button: WeaponButton) -> bool {
        self.next_state(button).is_some()
    }

    fn attack_name(&self, button: WeaponButton) -> Option<String> {
        self.next_state(button)
            .map(|(attack, _)| get_attack_name(attack))
    }
}
