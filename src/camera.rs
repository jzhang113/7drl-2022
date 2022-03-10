use rltk::Point;
use std::cmp::{max, min};

pub const VIEW_W: i32 = 79;
pub const VIEW_H: i32 = 50;
pub const MAP_W: i32 = 120;
pub const MAP_H: i32 = 120;

const MAX_X: i32 = const_max(MAP_W - VIEW_W, 0);
const MAX_Y: i32 = const_max(MAP_H - VIEW_H, 0);

const fn const_max(a: i32, b: i32) -> i32 {
    [a, b][(a < b) as usize]
}

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point,
}

impl Camera {
    pub fn update(&mut self, center: Point) {
        let top_left_x = max(center.x - VIEW_W / 2, 0);
        let top_left_y = max(center.y - VIEW_H / 2, 0);
        let origin_x = min(top_left_x, MAX_X);
        let origin_y = min(top_left_y, MAX_Y);
        self.origin = Point::new(origin_x, origin_y);
    }

    pub fn on_screen(&self, point: Point) -> bool {
        point.x >= self.origin.x
            && point.y >= self.origin.y
            && point.x < self.origin.x + VIEW_W
            && point.y < self.origin.y + VIEW_H
    }

    pub fn iter(&self) -> CameraIterator {
        CameraIterator::new(self.origin)
    }
}

pub struct CameraIterator {
    initial: Point,
    x: i32,
    y: i32,
}

impl CameraIterator {
    fn new(initial: Point) -> Self {
        Self {
            initial,
            x: initial.x - 1,
            y: initial.y,
        }
    }

    fn get_index(&self) -> usize {
        ((self.y * MAP_W) + self.x) as usize
    }
}

impl Iterator for CameraIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x < self.initial.x + VIEW_W {
            return Some(self.get_index());
        }

        self.x = self.initial.x;
        self.y += 1;
        if self.y < self.initial.y + VIEW_H {
            return Some(self.get_index());
        }

        None
    }
}
