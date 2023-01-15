use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use domain::entities::user_entity::UserEntity;
use domain::errors::add_user_error::AddUserError;
use domain::errors::add_user_error::AddUserPort;


pub struct FileRepository {
    file_path: String,
}

impl FileRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl AddUserPort for FileRepository {
    fn add_user(
        &self,
        last_name: &String,
        first_name: &String,
        email: &String,
    ) -> Result<UserEntity, AddUserError>{
        let content = format!("{};{};{}", last_name, first_name, email)
        let file_path = Path::new(&self.file_path);

        match file_path.parent() {
            Some(d) => match std::fs::create_dir_all(d) {
                Err(e) => {
                    let error_message = format!(
                        "{} {}",
                        d.display(),
                        e
                    );
                    let error = AddUserError::new(String::from(error_message));
                    return Err(error);
                }
                _ => (),
            }
        }
    }
}
