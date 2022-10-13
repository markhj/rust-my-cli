use std::{fs, io};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use rust_config_reader::{ConfigReader, Config, ConfigurationItem};

mod session;
mod package;
mod interpreter;

use session::Session;
use crate::interpreter::Interpreter;
use crate::package::Package;

fn main() {
    let config: Config = init();
    let mut session: Session = Session {
        using: None,
    };

    welcome(&config);
    create_session(&config, &mut session);

    loop {
        handle(&config, &mut session);
    }
}

fn create_session(config: &Config, session: &mut Session) {
    let default_package = config.group("main").unwrap().get_or("default_package", "");
    if !default_package.is_empty() {
        use_package(&config, session, default_package.as_str());
    }
}

fn welcome(config: &Config) {
    println!("{}", config.group("hello").unwrap().get("title").unwrap().value);
}

fn init() -> Config {
    let path_packages: &Path = Path::new("./packages");
    if !path_packages.exists() {
        fs::create_dir(path_packages).expect("Failed to create folder: packages");
    }

    let config_path: &Path = Path::new("./config");
    if !config_path.exists() {
        fs::copy(Path::new("./config-sample"), config_path).unwrap();
    }

    ConfigReader::read("./config", None).expect("Failed to load config file")
}

fn handle(config: &Config, session: &mut Session) {
    let mut input: String = String::new();

    print!("> ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed: &str = input.trim();
    let first: &str = trimmed.split(" ").collect::<Vec<&str>>()[0];
    match first {
        "exit" => std::process::exit(0),
        "packages" => list_packages(&config),
        "use" => set_using(&trimmed, &config, session),
        "help" => help(&config),
        "make:package" => make_package(&trimmed),
        "make:command" => make_command(&trimmed, &config, &session),
        _ => parse(&trimmed, &config, &session),
    };
}

fn make_command(input: &str, _config: &Config, session: &Session) {
    if session.using.is_none() {
        println!("Please select a package");
        return;
    }

    let command_name: &str = input.split(" ").collect::<Vec<&str>>()[1];
    let package: &Package = session.using.as_ref().unwrap();
    let path: String = package.get_path(format!("{}.scp", command_name));
    let command_path: &Path = Path::new(path.as_str());

    if command_path.exists() {
        println!("Command already exists");
        return;
    }

    fs::copy(
        Path::new("./res/stubs/new-command/command.scp"),
        command_path,
    ).expect("Failed to create new command");
}

fn make_package(input: &str) {
    let package_name: &str = input.split(" ").collect::<Vec<&str>>()[1];
    let dir: String = format!("./packages/{}", package_name);
    let new_package_path: &Path = Path::new(dir.as_str());
    if new_package_path.exists() {
        println!("Package already exists");
        return;
    }

    fs::create_dir(dir).expect("Failed to create package folder");

    let mut files: HashMap<&str, Vec<&str>> = HashMap::new();
    files.insert(".env.example", vec!(".env.example", ".env"));
    files.insert("_gitignore", vec!(".gitignore"));
    files.insert("hello.scp", vec!("hello.scp"));

    for (k, v) in files {
        for x in v {
            fs::copy(
                Path::new(format!("./res/stubs/new-package/{}", k).as_str()),
                Path::new(format!("./packages/{}/{}", package_name, x).as_str()),
            ).expect("Failed to create package");
        }
    }

    println!(
        "Package created!\nOpen the config file and add the \
        following under the [packages] list: {} = \"./packages/{}\"\
        \nYou will need to restart the application after.",
        package_name,
        package_name,
    );
}

fn help(_config: &Config) {
    println!("Help coming here...");
}

fn package_exists(config: &Config, name: &str) -> bool {
    config.group("packages").unwrap().keys().contains(&name.to_string())
}

fn set_using(input: &str, config: &Config, session: &mut Session) {
    use_package(&config, session, input.split(" ").collect::<Vec<&str>>()[1]);
}

fn use_package(config: &Config, session: &mut Session, package_name: &str) {
    if !package_exists(config, package_name) {
        println!("Package doesn't exist. You can add it in the configuration file.");
        return;
    }
    let path_to_package: PathBuf = Path::new(
        config.group("packages").unwrap().get(package_name).unwrap().as_str()
    ).to_owned();

    if !path_to_package.exists() {
        println!("Package not found at {:?}", path_to_package);
        return;
    }

    session.using = Some(Package {
        name: package_name.to_string(),
        path: path_to_package,
    });
    println!("Using package: {}", package_name);
}

fn list_packages(config: &Config) {
    config.group("packages").unwrap().for_each(|item: &ConfigurationItem| {
        println!("{}", item.key);
    })
}

fn parse(input: &str, _config: &Config, session: &Session) {
    if session.using.is_none() {
        println!("Select a package with command \"use <package>\"");
        return;
    }

    let command_name: &str = input.split(" ").collect::<Vec<&str>>()[0];
    let package: &Package = session.using.as_ref().unwrap();
    let path: String = package.get_path(format!("{}.scp", command_name));
    let command_path: &Path = Path::new(path.as_str());

    if !command_path.exists() {
        println!("Command {} not found in package {}", command_name, package.name);
        return;
    }

    Interpreter::run(command_path);
}
