pub mod files;
pub mod user;

use std::sync::RwLock;
use user::User;

pub struct Shell {
    input: String,
    cwd: RwLock<String>,
    user: User,
}

impl Shell {
    pub fn new_at_path(path: &str) -> Self {
        Shell {
            cwd: RwLock::new(path.into()),
            input: String::new(),
            user: User::default(),
        }
    }

    fn with_user(mut self, user: User) -> Self {
        self.user = user;
        self
    }

    pub fn new_in_home(user: User) -> Self {
        Self::new_at_path(&format!("/home/{}", user.get_name())).with_user(user)
    }

    fn get_prefix(&self) -> String {
        let path = self.cwd.read().unwrap();
        format!("[guest@deep-freezer:{}]$ ", *path)
    }
}
