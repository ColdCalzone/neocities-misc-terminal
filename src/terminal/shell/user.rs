pub struct User {
    name: String,
    path: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "guest".into(),
            path: "/bin/help:/bin/clear".into(),
        }
    }
}

impl User {
    fn from_name(name: &str) -> Self {
        match name {
            "cold" => User {
                name: "cold".into(),
                ..Default::default()
            },
            _ => User::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
