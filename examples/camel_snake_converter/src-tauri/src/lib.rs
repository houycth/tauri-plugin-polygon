use tauri::Manager;
use tauri_plugin_polygon::PolygonExt;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
    Mouse, Button
};
use arboard::Clipboard;
use std::time::Duration;
use std::thread;

fn simulate_ctrl_c() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Control, Press).unwrap();
    enigo.key(Key::Unicode('c'), Click).unwrap();
    thread::sleep(Duration::from_millis(20));
    enigo.key(Key::Control, Release).unwrap();
}

fn get_selected_content() -> Option<String> {
    if let Ok(mut clipboard) = Clipboard::new() {
        let history = clipboard.get_text();

        let _ = clipboard.clear();

        simulate_ctrl_c();

        let content = clipboard.get_text();

        if let Ok(content) = content {
            let content = content.trim();

            if let Ok(history) = history {
                let _ = clipboard.set_text(history);
            }

            if content.len() > 0 {
                return Some(content.to_string());
            }
        }
    }
    None
}

// Simulate ctrl + c. And get content from clipboard.
#[tauri::command]
fn get_content() -> String {
    get_selected_content().unwrap_or("".to_string())
}

#[tauri::command]
fn set_content(content: String) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo
        .text(&content)
        .unwrap();
}

// A trick to make the focus back to the window.
// Double click to select the word so that we can change it later.
#[tauri::command]
fn click_to_back_focus() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Left, Click).unwrap();
    enigo.button(Button::Left, Click).unwrap();
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    // Before we start. The fact we must know is that this application is a demostration of how to use the polygon plugin.
    // Do not this this app seriously.
    tauri::Builder::default()
        // Initialize the polygon plugin with a enmpty callback.
        // All events would emit to frontend, thus we do nothing here.
        // However, you can aslo use this callback to do something as you like.
        .plugin(tauri_plugin_polygon::init(|_app, _event| {}))
        .invoke_handler(tauri::generate_handler![get_content, set_content, click_to_back_focus])
        .setup(|app| {
            // We register a polygon here so we don't need to do it in the frontend.
            app.polygon().register("BUTTON").unwrap();
            // Skip taskbar if you like it.
            let win = app.get_webview_window("main").unwrap();
            win.set_skip_taskbar(true).unwrap();

            #[cfg(debug_assertions)]
            {
                win.open_devtools();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
