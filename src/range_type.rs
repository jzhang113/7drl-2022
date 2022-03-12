use rltk::Point;

#[derive(PartialEq)]
pub enum RangeType {
    Empty,
    Single,
    Square { size: i32 },
    Path { dest: rltk::Point },
    Custom { offsets: Vec<(i32, i32)> },
}

pub fn resolve_range_at(range: &RangeType, center: Point) -> Vec<Point> {
    let mut targets = Vec::new();

    match range {
        RangeType::Empty => {}
        RangeType::Single => {
            targets.push(center);
        }
        RangeType::Square { size } => {
            for x in center.x - size..=center.x + size {
                for y in center.y - size..=center.y + size {
                    targets.push(Point::new(x, y));
                }
            }
        }
        RangeType::Path { dest } => {
            targets = rltk::Bresenham::new(center, *dest).collect();
            targets.push(*dest);
        }
        RangeType::Custom { offsets } => {
            for (dx, dy) in offsets {
                targets.push(center + Point::new(*dx, *dy))
            }
        }
    }

    targets
}
