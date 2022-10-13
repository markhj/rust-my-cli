use std::fs;
use std::path::Path;
use std::process::Command;
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
            "exec" => {
                let str = Regex::new(r#"^\s*exec "([^"]*)"\s*$"#).unwrap().replace(line, "$1");
                let pts = str.split(" ").collect::<Vec<&str>>();

                let output = Command::new(pts[0])
                    .arg(pts[1])
                    .output();
                println!("status: {}", String::from_utf8_lossy(&output.unwrap().stdout));
            },
            _ => println!("Unknown control: {}", parts[0]),
        }
    }
}
