#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn route_system(data: &str) -> String {
    match data {
        "/home"     => format!("Welcome to Home"),
        "/about"    => format!("Welcome to About"),
        _           => format!("Oops, Page Not Found!"),
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![route_system])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
