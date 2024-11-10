use tauri::AppHandle;
use tauri::Runtime;

use crate::models::*;
use crate::PolygonExt;

#[tauri::command]
pub(crate) async fn register<R: Runtime>(app: AppHandle<R>, id: String) -> CommandResult {
    match app.polygon().register(&id) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn register_all<R: Runtime>(
    app: AppHandle<R>,
    polygons: Vec<String>,
) -> CommandResult {
    match app.polygon().register_all(polygons) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn remove<R: Runtime>(app: AppHandle<R>, id: String) -> CommandResult {
    match app.polygon().remove(&id) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn clear<R: Runtime>(app: AppHandle<R>) -> CommandResult {
    match app.polygon().clear() {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn show<R: Runtime>(app: AppHandle<R>, id: String) -> CommandResult {
    match app.polygon().show(&id) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn hide<R: Runtime>(app: AppHandle<R>, id: String) -> CommandResult {
    match app.polygon().hide(&id) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}

#[tauri::command]
pub(crate) async fn update<R: Runtime>(
    app: AppHandle<R>,
    id: String,
    points: Vec<(f64, f64)>,
) -> CommandResult {
    match app.polygon().update(&id, points) {
        Ok(()) => Response::ok(),
        Err(e) => Response::err(e),
    }
}
