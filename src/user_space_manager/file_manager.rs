use std::fs::{self, File};

pub fn create_file(file_name: &str) {
    File::create(format!("/users/[user name]/{}", file_name));
    println!("The file has made.");
}

pub fn remove_file(file_name: &str) {
    fs::remove_file(format!("/users/[user name]/{}", file_name));
    println!("The file has removed.")
}
