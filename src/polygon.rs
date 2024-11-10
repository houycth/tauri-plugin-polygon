use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use portable_atomic::AtomicF64;

pub type PolygonId = String;

#[derive(Debug)]
pub struct Polygon {
    id: PolygonId,
    /// The points of the polygon, at least 3 points needed
    points: AtomicPtr<Vec<(AtomicF64, AtomicF64)>>,
    /// Whether the polygon is currently being displayed
    display: AtomicBool,
    /// Whether the cursor is currently in the polygon
    cursor_in: AtomicBool,
}

impl Polygon {
    pub fn new(id: &str, points: &[(f64, f64)]) -> Self {
        let points: Vec<(AtomicF64, AtomicF64)> = points
            .iter()
            .map(|(x, y)| (AtomicF64::new(*x), AtomicF64::new(*y)))
            .collect();
        Self {
            id: id.to_string(),
            points: AtomicPtr::new(Box::into_raw(Box::new(points))),
            display: AtomicBool::new(false),
            cursor_in: AtomicBool::new(false),
        }
    }
    pub fn default(id: &str) -> Self {
        Self::new(id, &[(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)])
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn set_points(&self, points: &[(f64, f64)]) {
        let new_points: Vec<(AtomicF64, AtomicF64)> = points
            .iter()
            .map(|(x, y)| (AtomicF64::new(*x), AtomicF64::new(*y)))
            .collect();

        let old_points = self.points.load(Ordering::SeqCst);
        self.points
            .store(Box::into_raw(Box::new(new_points)), Ordering::SeqCst);

        // Points are saved as raw ptr so we need to free them manually
        drop(unsafe { Box::from_raw(old_points) });
    }
    pub fn points(&self) -> Vec<(f64, f64)> {
        unsafe {
            self.points
                .load(Ordering::SeqCst)
                .as_ref()
                .unwrap()
                .iter()
                .map(|(x, y)| (x.load(Ordering::SeqCst), y.load(Ordering::SeqCst)))
                .collect()
        }
    }
    pub fn hide(&self) {
        self.display.store(false, Ordering::SeqCst);
    }
    pub fn show(&self) {
        self.display.store(true, Ordering::SeqCst);
    }
    pub fn set_cursor_in(&self, in_polygon: bool) {
        self.cursor_in.store(in_polygon, Ordering::SeqCst);
    }
    pub fn cursor_in(&self) -> bool {
        self.cursor_in.load(Ordering::SeqCst)
    }
    pub fn display(&self) -> bool {
        self.display.load(Ordering::SeqCst)
    }
    pub fn distroy(&self) {
        // Points are saved as raw ptr so we need to free them manually
        let points = self.points.load(Ordering::SeqCst);
        self.points.store(std::ptr::null_mut(), Ordering::SeqCst);
        drop(unsafe { Box::from_raw(points) });
    }
}
