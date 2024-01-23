use super::model::User;
use std::result::Result;
use std::sync::Mutex;

pub struct UserService {
    users: Mutex<Vec<User>>,
}

impl UserService {
    pub fn new() -> UserService {
        UserService {
            users: Mutex::new(vec![]),
        }
    }

    pub fn add_user(&self, user: User) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.push(user);
        Ok(())
    }

    pub fn update_projects_contributed(&self, user_id: u32, new_count: u32) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            user.projects_contributed = new_count;
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    pub fn update_projects_created(&self, user_id: u32, new_count: u32) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
            user.projects_created = new_count;
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}
