mod init;
mod parse;

use clap::{crate_version, App, Arg};
use init::init;
use std::env::current_dir;

#[derive(Debug)]
pub struct ProjectConfig {
    pub language: String,
    pub name: String,
    pub folder_name: String,
    pub author: String,
    pub description: String,
    pub extras: Vec<String>,
    pub git: bool,
}

fn main() {
    // Set up the argument parsing and define all the arguments
    let args =
        App::new("init")
            .version(crate_version!())
            .author("William Henderson")
            .about("Initialise any project with one simple command.")
            .arg(
                Arg::with_name("language")
                    .required(true)
                    .index(1)
                    .help("Language to initialise the project for."),
            )
            .arg(Arg::with_name("name").index(2).help(
                "Name of the project. If unspecified, will be implied from the directory name.",
            ))
            .arg(
                Arg::with_name("author")
                    .short("a")
                    .long("author")
                    .takes_value(true)
                    .value_name("AUTHOR")
                    .help("Name of the project author."),
            )
            .arg(
                Arg::with_name("description")
                    .short("d")
                    .long("desc")
                    .takes_value(true)
                    .value_name("DESCRIPTION")
                    .help("A short description for the project."),
            )
            .arg(
                Arg::with_name("extras")
                    .short("e")
                    .long("extras")
                    .takes_value(true)
                    .value_name("EXTRA")
                    .multiple(true)
                    .help("Names of optional extras to add. These can be found by running `init <language> --list-extras`.")
            )
            .arg(
                Arg::with_name("list-extras")
                    .short("l")
                    .long("list-extras")
                    .help("List extras for the specified language.")
            )
            .arg(
                Arg::with_name("no-git")
                    .long("--no-git")
                    .help("Prevent init from initialising a Git repo.")
            )
            .get_matches();

    let cwd = current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let project_name = if args.is_present("name") && args.value_of("name").unwrap() != "." {
        args.value_of("name").unwrap()
    } else {
        &cwd
    };

    // Parse the arguments into a config object
    let config = ProjectConfig {
        language: args.value_of("language").unwrap().to_string(),
        name: project_name.to_string(),
        folder_name: args.value_of("name").unwrap_or(".").replace(" ", ""),
        author: args.value_of("author").unwrap_or("").to_string(),
        description: args.value_of("description").unwrap_or("").to_string(),
        extras: args
            .values_of("extras")
            .unwrap_or_default()
            .map(String::from)
            .collect(),
        git: !args.is_present("no-git"),
    };

    // Ensure that the language is valid or list supported languages
    let directory = parse::get_directory();
    if config.language != "list"
        && (!directory.contains(&config.language)
            || !config.language.chars().all(char::is_alphabetic))
    {
        println!("error: Language not supported, try `init list` to list supported languages");
        return;
    } else if config.language == "list" {
        println!("init currently supports the following languages:\n");
        for language in directory.dirs() {
            println!("  - {}", language.path);
        }
        return;
    }

    // If listing available extras, do that instead of initialising a project
    if args.is_present("list-extras") {
        let init_config = parse::parse_init_file(
            directory
                .get_file(format!("{}/init.json", config.language))
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )
        .unwrap();

        println!("available extras for {}:\n", config.language);
        for extra in init_config.extras {
            println!("  - {}", extra.name);
        }
    }

    init(&config, directory);
}
