use std::fs;

pub fn delete_user(user_name: &str) {
    fs::remove_dir_all(format!("/users/{}", user_name));
}
