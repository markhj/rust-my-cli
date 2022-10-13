use std::path::{PathBuf};
use regex::Regex;

pub struct Package {
    pub name: String,
    pub path: PathBuf,
}

impl Package {
    pub fn get_path(&self, file: String) -> String {
        format!(
            "{}/{}",
            Regex::new(r"(/\\)*$").unwrap().replace(self.path.to_str().unwrap(), ""), file
        )
    }
}
