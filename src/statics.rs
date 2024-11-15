use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::Ordering, Arc, OnceLock, RwLock},
};

use portable_atomic::AtomicF64;
use tauri::{Runtime, Window};

use crate::polygon::Polygon;

pub(crate) static REGISTERED_POLYGON: OnceLock<Arc<RwLock<HashMap<String, Polygon>>>> =
    OnceLock::new();
pub(crate) static REGISTERED_IDS: OnceLock<Arc<RwLock<HashSet<String>>>> = OnceLock::new();
pub(crate) static PHYSICAL_WIDTH: AtomicF64 = AtomicF64::new(0.0);

pub(crate) fn init<R: Runtime>(win: Window<R>) {
    let win_size = win.outer_size().unwrap();

    PHYSICAL_WIDTH.store(win_size.width as f64, Ordering::SeqCst);

    REGISTERED_IDS
        .set(Arc::new(RwLock::new(HashSet::new())))
        .unwrap();

    REGISTERED_POLYGON
        .set(Arc::new(RwLock::new(HashMap::new())))
        .unwrap();
}
