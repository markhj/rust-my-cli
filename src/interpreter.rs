use std::fs;
use std::path::Path;
use regex::Regex;

pub struct Interpreter;

impl Interpreter {
    pub fn run(path: &Path) {
        let data: String = fs::read_to_string(path)
            .expect("Failed to read file");

        for x in data.lines() {
            Interpreter::handle(x);
        }
    }
    fn handle(line: &str) {
        let parts: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

        match parts[0] {
            "echo" => println!("{}", Regex::new(r#"^\s*echo "([^"]*)"\s*$"#).unwrap().replace(line, "$1")),
            _ => println!("Unknown control: {}", parts[0]),
        }
    }
}
