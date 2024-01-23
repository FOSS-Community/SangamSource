use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub projects_contributed: u32,
    pub projects_created: u32,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            projects_contributed: 0,
            projects_created: 0,
        }
    }

    pub fn add_project_contribution(&mut self) {
        self.projects_contributed += 1;
    }

    pub fn add_project_creation(&mut self) {
        self.projects_created += 1;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, Username: {}, Email: {}, Projects Contributed: {}, Projects Created: {}",
            self.id, self.username, self.email, self.projects_contributed, self.projects_created
        )
    }
}