use log::{error, trace};
use portable_atomic::AtomicPtr;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::atomic::Ordering;
use tauri::Manager;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::error::Result;
use crate::utils::Convert;
use crate::view;

pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    f: PolygonCallback<R>,
) -> crate::Result<Polygon<R>> {
    Ok(Polygon {
        app_handle: app.clone(),
        callback: AtomicPtr::new(Box::into_raw(Box::new(f))),
    })
}

pub(crate) type PolygonCallback<R> =
    Box<dyn FnMut(&AppHandle<R>, crate::Event) + Send + Sync + 'static>;

/// Access to the Polygon APIs.
pub struct Polygon<R: Runtime> {
    pub app_handle: AppHandle<R>,
    callback: AtomicPtr<PolygonCallback<R>>,
}

impl<R: Runtime> Polygon<R> {
    pub(crate) fn emit(&self, app_handle: &AppHandle<R>, event: crate::Event) {
        let ptr = self.callback.load(Ordering::SeqCst);
        let mut callback = unsafe { Box::from_raw(ptr) };
        callback(app_handle, event);
        self.callback
            .store(Box::into_raw(callback), Ordering::SeqCst);
    }
    /// Register a default polygon with given id.
    ///
    /// Frequent calls to this function may cause performance issues.
    /// It is recommended to use `register_all` to register multiple polygons at once.
    ///
    /// # Errors
    /// This function will return an error if the `id` provided has already been registered.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().register("my-polygon")?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { register } from 'tauri-plugin-polygon-api';
    /// await register('my-polygon');
    /// ```
    pub fn register(&self, id: &str) -> Result<()> {
        trace!("register: {id}");
        match view::register(id.into()) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("register: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Register multiple polygons.
    ///
    /// # Errors
    /// This function will `not` return errors even if the `id` provided has already been registered.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().register(Vec::from(["my-polygon", "another-polygon"]))?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { registerAll } from 'tauri-plugin-polygon-api';
    /// await registerAll(['my-polygon', 'another-polygon']);
    /// ```
    pub fn register_all<S: AsRef<str> + Debug>(&self, ids: Vec<S>) -> Result<()> {
        trace!("register_all: {ids:?}");
        match view::register_all(ids.iter().map(|id| id.as_ref().to_string()).collect()) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("register_all: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Remove a polygon physically.
    ///
    /// After this function call ends, the specified polygon will be deleted physically,
    /// and needs to be re-registered before it can be used again.
    ///
    /// Frequent calls to this function may cause performance issues.
    /// It is recommended to use `hide` to disable the polygon logically.
    ///
    /// # Errors
    /// This function will return an error if the `id` provided can not be found.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().remove("my-polygon")?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { remove } from 'tauri-plugin-polygon-api';
    /// await remove('my-polygon');
    /// ```
    pub fn remove(&self, id: &str) -> Result<()> {
        trace!("remove: {id}");
        match view::remove(&id) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("remove: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Enable the polygon by given id.
    ///
    /// # Errors
    /// This function will return an error if the `id` provided can not be found.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().show("my-polygon")?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { show } from 'tauri-plugin-polygon-api';
    /// await show('my-polygon');
    /// ```
    pub fn show(&self, id: &str) -> Result<()> {
        trace!("show: {id}");
        match view::show(&id) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("show: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Disable the polygon logically by given id.
    ///
    /// # Errors
    /// This function will return an error if the `id` provided can not be found.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().hide("my-polygon")?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { hide } from 'tauri-plugin-polygon-api';
    /// await hide('my-polygon');
    /// ```
    pub fn hide(&self, id: &str) -> Result<()> {
        trace!("hide: {id}");

        self.app_handle
            .get_webview_window("main")
            .unwrap()
            .set_ignore_cursor_events(true)
            .unwrap();

        match view::hide(&id) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("hide: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Update vertices of the polygon by given id.
    /// Within these points, mouse events will not go through.
    ///
    /// # Notice
    /// 1. All positions should be converted to a `percentage based on the screen width`.
    ///    Position from 0 to 1, 0.1 means 10% of the `screen width`.
    /// 2. At least `3` points are required.
    /// 3. The order in which you define the points matters and can result in different shapes.
    ///
    /// # Errors
    /// This function will return an error if the `id` provided can not be found.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().update("my-polygon", vec![(0.0, 0.0), (0.1, 0.0), (0.1, 0.1), (0.0, 0.1)])?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { update } from 'tauri-plugin-polygon-api';
    ///
    /// await update('my-polygon', {
    ///     id: "EXAMPLE",
    ///     polygon: [
    ///       [0, 0],
    ///       [0.1, 0],
    ///       [0.1, 0.1],
    ///       [0, 0.1]
    ///     ]
    /// })
    /// ```
    pub fn update(&self, id: &str, points: Vec<(f64, f64)>) -> Result<()> {
        trace!("update: {id} - {points:?}");
        match view::update(
            &id,
            &points
                .iter()
                .map(|(x, y)| Convert::from_viewport(*x, *y))
                .collect::<Vec<(f64, f64)>>(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("update: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    /// Clear all polygons physically.
    ///
    /// # Example
    /// ```no_run
    /// // backend with rust
    /// app.polygon().clear()?;
    /// ```
    /// ```javascript
    /// // frontend with js
    /// import { clear } from 'tauri-plugin-polygon-api';
    /// await clear();
    /// ```
    pub fn clear(&self) -> Result<()> {
        trace!("clear");
        match view::clear() {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("clear: {e}");
                self.emit(&self.app_handle, crate::Event::Error(e.clone()));
                Err(e)
            }
        }
    }
    pub(crate) fn destroy(&self) -> Result<()> {
        let ptr = self.callback.load(Ordering::SeqCst);
        self.callback.store(std::ptr::null_mut(), Ordering::SeqCst);
        drop(unsafe { Box::from_raw(ptr) });
        Ok(())
    }
}
