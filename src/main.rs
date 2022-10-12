use std::{fs, io};
use std::io::Write;
use std::path::Path;
use rust_config_reader::{ConfigReader, Config, ConfigurationItem};

mod session;
use session::Session;

// Convert using: String to being a Package struct

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
    let directives_path : &Path = Path::new("./directives");
    if !directives_path.exists() {
        fs::create_dir(directives_path).expect("Failed to create folder: directives");
    }

    let config_path : &Path = Path::new("./config");
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
    let first: &str = trimmed.clone().split(" ").collect::<Vec<&str>>()[0];
    match first {
        "exit" => std::process::exit(0),
        "packages" => list_packages(&config),
        "use" => set_using(&trimmed, &config, session),
        _ => parse(&trimmed, &config, &session),
    };
}

fn package_exists(config: &Config, name: &str) -> bool {
    config.group("packages").unwrap().keys().contains(&name.to_string())
}

fn set_using(input: &str, config: &Config, session: &mut Session) {
    use_package(&config, session, input.split(" ").collect::<Vec<&str>>()[1]);
}

fn use_package(config: &Config, session: &mut Session, space: &str) {
    if !package_exists(config, space) {
        println!("Package does not exist. You can add it in the configuration file.");
        return;
    }
    session.using = Some(space.to_string());
    println!("Using package: {}", space);
}

fn list_packages(config: &Config) {
    config.group("packages").unwrap().for_each(|item: &ConfigurationItem| {
        println!("{}", item.key);
    })
}

fn parse(input: &str, config: &Config, session: &Session) {
    if session.using.is_none() {
        println!("Select a space with command \"use <space>\"");
        return;
    }
    println!("Parsing: {} USING: {}", input, session.using.as_ref().unwrap());
}
