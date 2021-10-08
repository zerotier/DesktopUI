extern crate tinyfiledialogs;

use tinyfiledialogs::{YesNo, MessageBoxIcon, DefaultColorValue};

fn main() {

    let choice = tinyfiledialogs::message_box_yes_no("hello", "yes or no?",
                                                     MessageBoxIcon::Question, YesNo::No);

    let user_input: String;
    match tinyfiledialogs::input_box("Enter user name", "Username:", "") {
        Some(input) => user_input = input,
        None => user_input = "null".to_string(),
    }

    let user_input_2: String;
    match tinyfiledialogs::input_box("Re-enter user name", "Username:", &user_input) {
        Some(input) => user_input_2 = input,
        None => user_input_2 = "null".to_string(),
    }

    let password_input: String;
    match tinyfiledialogs::password_box("Enter password", "Password:") {
        Some(input) => password_input = input,
        None => password_input = "null".to_string(),
    }

    let save_file: String;
    match tinyfiledialogs::save_file_dialog("Save", "password.txt") {
        Some(file) => save_file = file,
        None => save_file = "null".to_string(),
    }

    let open_file: String;
    match tinyfiledialogs::open_file_dialog("Open", "password.txt", None) {
        Some(file) => open_file = file,
        None => open_file = "null".to_string(),
    }

    let folder: String;
    match tinyfiledialogs::select_folder_dialog("Select folder", "") {
        Some(result) => folder = result,
        None => folder = "null".to_string(),
    }

    let color: String;
    match tinyfiledialogs::color_chooser_dialog("Choose a Color", DefaultColorValue::Hex("#FF0000")) {
        Some((hex_result, _rgb)) => color = hex_result,
        None => color = "null".to_string(),
    }

    println!("Choice {:?}", choice);
    println!("User input {:?}", user_input);
    println!("User input 2 {:?}", user_input_2);
    println!("Password input {:?}", password_input);
    println!("Save file {:?}", save_file);
    println!("Open file {:?}", open_file);
    println!("folder {:?}", folder);
    println!("color {:?}", color);
}
