use std::{collections::HashMap, sync::LazyLock};

use macro_hash::hash;

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    home_dir: Option<String>,
    path: String,
    password: Option<u64>,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "guest".into(),
            home_dir: None,
            path: "/bin/".into(),
            password: None,
        }
    }
}

pub static USERS: LazyLock<HashMap<&str, User>> = LazyLock::new(|| {
    HashMap::from([
        (
            "cold",
            User {
                name: "cold".into(),
                password: Some(hash!("TestingPasswordThisIsntGoingIntoProduction")),
                home_dir: Some("/root/".into()),
                ..Default::default()
            },
        ),
        (
            "guest",
            User {
                name: "guest".into(),
                password: None,
                ..Default::default()
            },
        ),
    ])
});

impl User {
    pub fn check_password(&self, pass: Option<u64>) -> bool {
        (self.password.is_none() && pass.is_none())
            || self.password.zip(pass).is_some_and(|(p, po)| p == po)
    }

    pub fn has_password(name: &str) -> bool {
        USERS.get(name).is_some_and(|x| x.password.is_some())
    }

    pub fn from_name(name: &str) -> User {
        if let Some(user) = USERS.get(name) {
            user
        } else {
            USERS.get("guest").unwrap()
        }
        .clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn home_directory(&self) -> String {
        self.home_dir
            .clone()
            .unwrap_or(format!("/home/{}", self.get_name()))
    }
}

#[derive(Debug)]
pub enum SignInError {
    NoUser,
    IncorrectPassword,
}
