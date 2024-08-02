use super::file_manager;
use super::folder_manager;

pub struct USER {
    name: String,
}

impl USER {
    pub fn new(name: &str) -> Self {
        let name = name.to_string();
        let user = USER {
            name
        };

        user
    }

    pub fn create_user_space(user_name: &str) {
        folder_manager::create_folder(user_name);
    }
}
