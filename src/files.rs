use std::fs::{read_dir, read_to_string, File};
use std::io::Write;
use std::process::exit;

// Auxiliars
pub fn get_dir() -> String {
    let paths = read_dir("/sys/class/backlight").unwrap();
    let mut result = String::from("empty");
    for entry in paths {
        let temp = entry.unwrap().path();
        if temp.is_dir() {
            result = temp.display().to_string();
        }
    }
    return result;
}

fn write_file(content: String, path: &str) {
    let mut file = File::create(path).unwrap();
    match file.write(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error writing {path}: {e}");
            exit(3);
        }
    }
}

fn read_file(path: &str) -> String {
    match read_to_string(path) {
        Ok(content) => return content,
        Err(e) => {
            println!("Error reading {path}: {e}");
            exit(2);
        }
    }
}

// Get the max_brightness and brightness info
pub fn get_info(brightness_path: &str, max_brightness_path: &str) -> (i16, i16) {
    let max_brightness = read_file(max_brightness_path);
    let current_brightness = read_file(brightness_path);
    return (
        max_brightness.trim().parse().unwrap(),
        current_brightness.trim().parse().unwrap(),
    );
}

// Restore the last brightness
pub fn restore(home_path: &str, brightness_path: &str) {
    let path = format!("{home_path}/.config/rustlight");
    let last_brightness = read_file(&path);
    write_file(last_brightness, brightness_path);
}

// Save the brightness value in /sys/class/backlight and .config
pub fn save_brightness(home_path: &str, brightness_path: &str, content: String) {
    let path = format!("{home_path}/.config/rustlight");
    write_file(content.clone(), &path);
    write_file(content.clone(), brightness_path);
}
