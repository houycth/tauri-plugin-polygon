# Tauri Plugin polygon

![License](https://img.shields.io/badge/License-MIT-blue.svg)
[![crates.io](https://img.shields.io/badge/crates.io-0.1.0-darkgreen.svg)](https://crates.io/crates/tauri-plugin-polygon)


A plugin for [tauri@v2](https://tauri.app/) to achieve click-through of the tauri main window by allowing developers to define polygons, thus customizing the mouse response area.

> Note that this plugin is only available for `full-screen` (normally `transparent background`) applications.

## Platform Supported

| Platform | Supported |
| -------- | :-------: |
| Windows  |    ✅     |
| Linux    |    ✅     |
| macOS    |    ✅     |
| Android  |    ❌     |
| iOS      |    ❌     |

## Install

Before using the library, you may need to learn more information about [tauri](https://tauri.app/start/).

```bash
# You need to create a tauri project first
# and then
npm install tauri-plugin-polygon-api

# You may need to `cd src-tauri` first
cargo add tauri-plugin-polygon
```

## Usage

_Go with [examples](https://github.com/houycth/tauri-plugin-polygon/tree/main/examples)._

### Configuration
Before using this plugin, we need to make some changes to `tauri.conf.json`, `html` and `src-tauri\capabilities\default.json`, so that we can build a full-screen and transparent background application, and invoke commands from the JS context.

```json5
// tauri.conf.json
"app": {
    "windows": [
      {
        // ...
        "alwaysOnTop": true,
        "transparent": true,
        "fullscreen": true
        // ...
      }
    ]
  },
```
```html
<!-- index.html -->
<!DOCTYPE html>
<html>
  <body style="background: transparent; width: 100vw; height: 100vh; overflow: hidden;">
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
  </body>
</html>
```
```json5
// src-tauri\capabilities\default.json
{
    // ...
    "windows": ["main"],
    "permissions": [
        // ...
        // Depends on your needs
        "polygon:allow-register",
        "polygon:allow-register-all",
        "polygon:allow-show",
        "polygon:allow-hide",
        "polygon:allow-remove",
        "polygon:allow-update",
        "polygon:allow-clear"
    ]
    // ...
}
```

### Initialization

```rust
use tauri::AppHandle;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_polygon::init(|_app, _event| {}))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Call from Rust

Learn more api about [tauri-plugin-polygon](https://docs.rs/tauri-plugin-polygon).

#### Example

```rust
use tauri::AppHandle;
use tauri_plugin_polygon::PolygonExt;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_polygon::init(|app, event| {
            // Do nothing beyond match block, otherwise, thread stack overflow would occur.
            // Some Event will be passed here.
            match event {
                tauri_plugin_polygon::Event::LeftClick {x, y} => {
                    println!("Left button clicked at ({x}, {y})");
                    // Update polygon's points
                    app.polygon().update("my-polygon",
                        vec![(0.0, 0.0), (0.1, 0.0), (0.1, 0.1), (0.1, 0.0)]
                    );
                    // Enable the polygon
                    app.polygon().show("my-polygon");
                }
                // Some other events
                _ => {}
            }
        }))
        .setup(|app| {
            // Register a polygon when application setup
            app.polygon().register("my-polygon").unwrap();

            // You may need to open devtools for debugging
            #[cfg(debug_assertions)]
            {
                let win = app.get_webview_window("main").unwrap();
                win.open_devtools();
            }
        }
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Call from Javascript

#### Example

```js
import { polygon } from 'tauri-plugin-polygon-api';

const SOME_ID = "my-polygon";

polygon.register(SOME_ID);

polygon.on("LeftClick", async payload => {
    await polygon.show(SOME_ID);
    // Update polygon's points
    // Note:
    // 1. Percentage is used here.
    // 2. At least 3 points needed.
    // 3. Order of points matters.
    await polygon.update(SOME_ID, [
        [0, 0],[0.1, 0],[0.1, 0.1],[0.1, 0]
    ]);
    // Make the polygon 'visible' (We do not really see the polygon).
    await polygon.show(SOME_ID);
})
```

> Notice: 
> 1. Events would be emmitd to `webview` and the `closure`(provided in init function)mouse event `triggered in unregistered areas`. As for registered areas, handle it by frendend itself.
> 2. Position from 0 to 1, 0.1 means 10% of the `screen` (which is fullscreen as we set before) `width`.
> 3. Order of points matters.
> 4. We can get the actual(logical) position by `window.screen.width * position.x` and `window.screen.width * position.y`.