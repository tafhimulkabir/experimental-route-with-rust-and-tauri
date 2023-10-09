#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{self, Read};

fn read_from_html(path: &str) -> io::Result<String> {
    // Open the HTML file with error handling
    let mut file = File::open(path)?;
    
    // Read the content of the file into a String
    let mut html_content = String::new();
    file.read_to_string(&mut html_content)?;
    
    Ok(html_content)
}

fn load_page(path: &str) -> String {
    // Handle potential errors from reading the HTML file
    match read_from_html(path) {
        Ok(content) => content,
        Err(err) => format!("Error: {}", err),
    }
}

#[tauri::command]
fn route_system(data: Option<&str>) -> String {
    // Match data and load the corresponding page
    match data {
        Some("/home") => load_page("../page/home.html"),
        Some("/about") => load_page("../page/about.html"),
        _ => format!("Oops, Page Not Found!"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![route_system])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
