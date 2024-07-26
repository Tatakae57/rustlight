use std::env::{args, var};
mod files;

fn help() {
    println!("Usage:");
    println!("set (value): Set brightness value.");
    println!("add (value): Add value.");
    println!("sub (value): Sub value.");
    println!("restore: Restore the last brightness.");
    println!("help: Show this message.");
    println!("\nThe values can be integers (10, 50, 60) or percentages (10%, 20%, 30%).");
}

fn verify_limits(value: String, max: i16) -> i16 {
    let result: i16;

    // Extract percentage/value
    if value.ends_with("%") {
        let temp: i16 = value.trim_end_matches("%").parse().unwrap();
        result = ((max as f64) * (temp as f64 / 100.0)) as i16;
    } else {
        let temp = value.parse().unwrap();
        result = temp;
    }

    // Returns
    if result > max {
        return max;
    } else if result < 0 {
        return 0;
    } else {
        return result;
    }
}

fn set_value(value: String, max: i16) -> String {
    // Get value
    let mut set_value: i16 = verify_limits(value, max);
    if set_value > max {
        set_value = max;
    } else if set_value < 0 {
        set_value = 0;
    }
    return set_value.to_string();
}

fn add_value(value: String, add: bool, max_value: i16, current_value: i16) -> String {
    let (mut addition, result): (i16, i16);

    // Get value.
    addition = verify_limits(value, max_value);

    // Verify is an substraction
    if add == false {
        addition = -addition;
    }

    // Verify if the addition exceeds the maximum or minimum brightness
    if current_value + addition > max_value {
        result = max_value;
    } else if current_value + addition < 0 {
        result = 0;
    } else {
        result = current_value + addition;
    }

    return result.to_string();
}

fn verify_type(args: Vec<String>, brightness_dir: String) {
    let home = var("HOME").unwrap();
    let (option_type, mut result) = (String::from(&args[1]), String::new());

    let (max_value, current_value): (i16, i16) = files::get_info(
        &format!("{brightness_dir}/brightness"),
        &format!("{brightness_dir}/max_brightness"),
    );

    let mut is_change: bool = false;

    // Verify first parameter
    match option_type.as_str() {
        // Add and Substract
        "add" => {
            result = add_value(args[2].clone(), true, max_value, current_value);
            is_change = true
        }
        "sub" => {
            result = add_value(args[2].clone(), false, max_value, current_value);
            is_change = true
        }
        "help" => help(),

        // Set and reset
        "set" => {
            result = set_value(args[2].clone(), max_value);
            is_change = true
        }
        "restore" => files::restore(&home, "/sys/class/backlight/intel_backlight/brightness"),

        // Invalid
        invalid => println!("Invalid argument: {invalid}"),
    }

    // Save the brightness

    if is_change {
        files::save_brightness(
            &home,
            "/sys/class/backlight/intel_backlight/brightness",
            result,
        );
    }
}

fn main() {
    // Get brightness dir
    let brightness_dir = files::get_dir();

    // Get arguments
    let args: Vec<String> = args().collect();
    if args.len() > 1 && args.len() <= 3 {
        verify_type(args, brightness_dir);
    } else if args.len() > 3 {
        println!("Too many arguments.");
    } else {
        println!("Too few arguments.");
    }
}
