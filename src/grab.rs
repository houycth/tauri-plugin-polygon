use std::{
    fmt::Display,
    sync::{atomic::Ordering, RwLock},
    thread,
    time::Instant,
};

use log::{error, trace};
use portable_atomic::AtomicF64;
use rdev;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter, Manager, Runtime, Window};

use crate::statics::REGISTERED_POLYGON;
use crate::utils::Convert;
use crate::view;
use crate::PolygonExt;

/// Saves physical pixel number
static MOUSE_X: AtomicF64 = AtomicF64::new(0.0);
/// Saves physical pixel number
static MOUSE_Y: AtomicF64 = AtomicF64::new(0.0);

#[derive(Debug)]
pub enum Event {
    LeftClick { x: f64, y: f64 },
    DoubleClick { x: f64, y: f64 },
    RightClick { x: f64, y: f64 },
    Drag { from: Position, to: Position },
    MouseMove { x: f64, y: f64 },
    Wheel { x: f64, y: f64 },
    Error(crate::Error),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::LeftClick { .. } => write!(f, "POLYGON_LEFT_CLICK"),
            Event::DoubleClick { .. } => write!(f, "POLYGON_DOUBLE_CLICK"),
            Event::RightClick { .. } => write!(f, "POLYGON_RIGHT_CLICK"),
            Event::MouseMove { .. } => write!(f, "POLYGON_MOUSE_MOVE"),
            Event::Wheel { .. } => write!(f, "POLYGON_WHEEL"),
            Event::Drag { .. } => write!(f, "POLYGON_DRAG"),
            Event::Error(..) => write!(f, "POLYGON_ERROR"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position {{x: {}, y: {} }}", self.x, self.y)
    }
}

fn get_mouse_position() -> (f64, f64) {
    Convert::to_viewport(
        MOUSE_X.load(Ordering::SeqCst),
        MOUSE_Y.load(Ordering::SeqCst),
    )
}

fn set_mouse_position(x: f64, y: f64) {
    MOUSE_X.store(x, Ordering::SeqCst);
    MOUSE_Y.store(y, Ordering::SeqCst);
}

fn emit<R: Runtime>(handle: &AppHandle<R>, event: Event) {
    match event {
        Event::LeftClick { x, y }
        | Event::RightClick { x, y }
        | Event::DoubleClick { x, y }
        | Event::MouseMove { x, y }
        => {
            trace!("emit event: {event:?}");
            let _ = handle.emit(
                &event.to_string(),
                json!({
                    "position": {
                        "x": x,
                        "y": y,
                    }
                }),
            );
            handle.polygon().emit(handle, event);
        }
        Event::Wheel { x, y } => {
            trace!("emit event: {event:?}");
            let _ = handle.emit(
                &event.to_string(),
                json!({
                    "delta": {
                        "x": x,
                        "y": y,
                    }
                }),
            );
            handle.polygon().emit(handle, event);
        }
        Event::Drag { from, to } => {
            trace!("emit event: {event:?}");
            let _ = handle.emit(
                &event.to_string(),
                json!({
                    "from": {
                        "x": from.x,
                        "y": from.y
                    },
                    "to": {
                        "x": to.x,
                        "y": to.y
                    }
                }),
            );
            handle.polygon().emit(handle, event);
        }
        Event::Error(e) => {
            let err = e.clone();
            let evt = Event::Error(e);

            error!("emit event: {evt}, error: {err}",);
            let _ = handle.emit(
                &evt.to_string(),
                json!({
                    "error": err.to_string()
                }),
            );
            handle.polygon().emit(handle, evt);
        }
    }
}

pub fn init<R: Runtime>(win: Window<R>) {
    let last_click_time = RwLock::new(Instant::now());
    let press_time = RwLock::new(Instant::now());
    let press_pos = RwLock::new(Position { x: 0.0, y: 0.0 });
    let win_clone_01 = win.clone();
    let win_clone_02 = win.clone();

    let thread_handle = thread::Builder::new()
        .name("polygon-grab".to_string())
        .spawn(move || {
            let result = rdev::grab(move |ev| match ev.event_type {
                rdev::EventType::ButtonPress(rdev::Button::Left) => {
                    let mut press_time = press_time.write().unwrap();
                    *press_time = Instant::now();

                    let mut press_pos = press_pos.write().unwrap();
                    let (x, y) = get_mouse_position();
                    press_pos.x = x;
                    press_pos.y = y;

                    Some(ev)
                }
                rdev::EventType::ButtonRelease(rdev::Button::Left) => {
                    let elapsed = press_time.read().unwrap().elapsed().as_millis();
                    let handle = win.app_handle();

                    let polygons = match view::cursor_in() {
                        Ok(v) => v,
                        Err(e) => {
                            emit(&handle, Event::Error(e));
                            return Some(ev);
                        }
                    };
                    // if click/drag triggered in a registered area, handle it by frontend self
                    // otherwise, send it to frontend.
                    if polygons.len() == 0 {
                        let (x, y) = get_mouse_position();
                        let press_pos = press_pos.read().unwrap();
                        // we assume it's a drag if the elapsed is more than 150ms or the mouse position has changed,
                        // otherwise it's a click.
                        if elapsed < 150 || (press_pos.x == x && press_pos.y == y) {
                            let mut last_click_time = last_click_time.write().unwrap();
                            let last_click_elapsed = last_click_time.elapsed().as_millis();
                            if last_click_elapsed < 400 {
                                emit(&handle, Event::DoubleClick { x, y });
                            } else {
                                emit(&handle, Event::LeftClick { x, y });
                            }
                            *last_click_time = Instant::now();
                        } else {
                            let (x, y) = get_mouse_position();
                            emit(
                                &handle,
                                Event::Drag {
                                    from: press_pos.clone(),
                                    to: Position { x, y },
                                },
                            );
                        }
                    }
                    Some(ev)
                }
                rdev::EventType::ButtonRelease(rdev::Button::Right) => {
                    let polygons = match view::cursor_in() {
                        Ok(v) => v,
                        Err(e) => {
                            let handle = win.app_handle();
                            emit(&handle, Event::Error(e));
                            return Some(ev);
                        }
                    };
                    // if click/drag triggered in a registered area, handle it by frontend self
                    if polygons.len() == 0 {
                        let (x, y) = get_mouse_position();
                        let handle = win.app_handle();
                        emit(&handle, Event::RightClick { x, y });
                    }
                    Some(ev)
                }
                rdev::EventType::MouseMove { x, y } => {
                    set_mouse_position(x, y);

                    let registered = REGISTERED_POLYGON.get().unwrap().read().unwrap();

                    let mut at_least_one = false;
                    for polygon in registered.values() {
                        if view::pos_contained(polygon, x, y) {
                            polygon.set_cursor_in(true);
                            at_least_one = true;
                        } else {
                            polygon.set_cursor_in(false);
                        }
                    }
                    // we have no way to ignore cursor event separately for each polygon
                    // so we should not ignore it if there is at least one polygon in the registered area
                    if at_least_one {
                        win.set_ignore_cursor_events(false).unwrap();
                    } else {
                        win.set_ignore_cursor_events(true).unwrap();
                    }
                    let handle = win.app_handle();
                    let mouse_pos = get_mouse_position();
                    emit(&handle, Event::MouseMove { x: mouse_pos.0, y: mouse_pos.1 });
                    Some(ev)
                }
                rdev::EventType::Wheel { delta_x, delta_y } => {
                    let handle = win.app_handle();
                    emit(&handle, Event::Wheel { x: delta_x as f64, y: delta_y as f64 });
                    Some(ev)
                }
                _ => Some(ev),
            });

            if let Err(e) = result {
                error!("Failed to grab events: {e:?}");
                emit(
                    win_clone_02.app_handle(),
                    Event::Error(crate::Error::PluginInitializationError(format!("{e:?}"))),
                );
            }
        });

    if let Err(e) = thread_handle {
        error!("Failed to start a event grab thread: {e:?}");
        let app_handle = win_clone_01.app_handle();
        emit(
            app_handle,
            Event::Error(crate::Error::PluginInitializationError(e.to_string())),
        );
    }
}