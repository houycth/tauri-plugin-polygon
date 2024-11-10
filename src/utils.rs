use std::sync::atomic::Ordering;

use crate::statics::PHYSICAL_WIDTH;

/// All point data conversions are based on the physical pixel width of the screen
/// to provide a unified standard.
/// Therefore, when the front end receives the data,
/// it needs to be multiplied by window.screen.width to obtain the logical pixel points.
pub struct Convert {}

impl Convert {
    pub fn from_viewport(x: f64, y: f64) -> (f64, f64) {
        let physical_width = get_physical_width();

        (x * physical_width, y * physical_width)
    }
    pub fn to_viewport(x: f64, y: f64) -> (f64, f64) {
        let physical_width = get_physical_width();

        let mut x = x / physical_width;
        if x < 0.0 {
            x = 0.0;
        }
        if x > 1.0 {
            x = 1.0;
        }

        let mut y = y / physical_width;
        if y < 0.0 {
            y = 0.0;
        }
        if y > 1.0 {
            y = 1.0;
        }

        (x, y)
    }
}

pub fn get_physical_width() -> f64 {
    PHYSICAL_WIDTH.load(Ordering::SeqCst)
}

pub fn is_point_in_polygon(polygon: &[(f64, f64)], point: (f64, f64)) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let epsilon = 1e-10;

    for i in 0..n {
        let j = (i + 1) % n;
        let xi = polygon[i].0;
        let yi = polygon[i].1;
        let xj = polygon[j].0;
        let yj = polygon[j].1;

        let point_x = point.0;
        let point_y = point.1;

        let intersect = ((yi > point_y) != (yj > point_y))
            && (point_x < (xj - xi) * (point_y - yi) / (yj - yi) + xi - epsilon);
        if intersect {
            inside = !inside;
        }
    }

    inside
}
