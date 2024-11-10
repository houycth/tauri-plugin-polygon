const COMMANDS: &[&str] = &[
  "register",
  "register_all",
  "update",
  "hide",
  "show",
  "remove",
  "clear"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
