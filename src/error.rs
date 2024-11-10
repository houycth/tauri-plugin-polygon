use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::polygon;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Deserialize, Serialize, Clone)]
pub enum Error {
    #[error(
        "Not Initialized. Call tauri_wherever::init(app_handle) first when setup a tauri app."
    )]
    NotInitialized,
    #[error("Polygon with id [{0}] not found.")]
    PolygonNotFound(String),
    #[error("Polygon with id [{0}] already exists.")]
    PolygonExists(String),
    #[error("At least 3 points needed but got {0}.")]
    PointsNotEnough(usize),
    #[error("Can not read/write cache. {0}")]
    LockError(String),
    #[error("Failed to initialize plugin. {0}")]
    PluginInitializationError(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::PluginInitializationError(format!("{error}"))
    }
}

impl<'a> From<PoisonError<RwLockWriteGuard<'a, HashSet<String>>>> for Error {
    fn from(error: PoisonError<RwLockWriteGuard<'a, HashSet<String>>>) -> Self {
        Error::LockError(format!("{error}"))
    }
}

impl<'a> From<PoisonError<RwLockReadGuard<'a, HashMap<String, polygon::Polygon>>>> for Error {
    fn from(error: PoisonError<RwLockReadGuard<'a, HashMap<String, polygon::Polygon>>>) -> Self {
        Error::LockError(format!("{error}"))
    }
}

impl<'a> From<PoisonError<RwLockWriteGuard<'a, HashMap<String, polygon::Polygon>>>> for Error {
    fn from(error: PoisonError<RwLockWriteGuard<'a, HashMap<String, polygon::Polygon>>>) -> Self {
        Error::LockError(format!("{error}"))
    }
}
