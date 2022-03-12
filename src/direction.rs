use derivative::Derivative;
use rltk::Point;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Derivative)]
#[derivative(Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn get_direction_towards(from: Point, goal: Point) -> Option<Direction> {
        let dx = goal.x - from.x;
        let dy = goal.y - from.y;

        if dx.abs() > dy.abs() {
            match dx.signum() {
                1 => Some(crate::Direction::E),
                -1 => Some(crate::Direction::W),
                _ => unreachable!(), // if dx.signum is 0, dx = 0, but we can't be in this branch in that case
            }
        } else {
            match dy.signum() {
                1 => Some(crate::Direction::S),
                -1 => Some(crate::Direction::N),
                _ => None,
            }
        }
    }

    pub fn point_in_direction(from: Point, direction: Direction) -> Point {
        match direction {
            Direction::N => from + Point::new(0, -1),
            Direction::E => from + Point::new(1, 0),
            Direction::S => from + Point::new(0, 1),
            Direction::W => from + Point::new(-1, 0),
        }
    }
}
