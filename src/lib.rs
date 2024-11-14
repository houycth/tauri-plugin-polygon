use tauri::AppHandle;
#[cfg(desktop)]
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod commands;
mod desktop;
mod error;
mod grab;
mod models;
mod polygon;
mod statics;
mod utils;
mod view;
mod thread_pool;

pub use desktop::Polygon;
pub use error::{Error, Result};
pub use grab::Event;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the polygon APIs.
pub trait PolygonExt<R: Runtime> {
    fn polygon(&self) -> &Polygon<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PolygonExt<R> for T {
    fn polygon(&self) -> &Polygon<R> {
        self.state::<Polygon<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime, F: FnMut(&AppHandle<R>, crate::Event) + Send + Sync + 'static>(
    f: F,
) -> TauriPlugin<R> {
    Builder::new("polygon")
        .invoke_handler(tauri::generate_handler![
            commands::register,
            commands::register_all,
            commands::remove,
            commands::show,
            commands::hide,
            commands::update,
            commands::clear
        ])
        .setup(|app, api| {
            let polygon = desktop::init(app, api, Box::new(f))?;
            app.manage(polygon);
            Ok(())
        })
        .on_window_ready(move |win| {
            if win.label() == "main" {
                statics::init(win.clone());
                grab::init(win.clone());
            }
        })
        .on_drop(|app| {
            // Clear all polygon in cache
            let _ = app.state::<Polygon<R>>().inner().clear();
            // drop callback
            let _ = app.state::<Polygon<R>>().inner().destroy();
        })
        .build()
}
