use std::fs;

pub fn create_folder(folder_name: &str) {
    fs::create_dir(format!("users/{}", folder_name));
    println!("The folder has made.")
}

pub fn remove_folder(folder_name: &str) {
    fs::remove_dir(format!("users/{}", folder_name));
    println!("The folder has removed.");
}
