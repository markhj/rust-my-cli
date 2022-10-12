
pub struct Session {
    pub using: Option<String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            using: None,
        }
    }
}


