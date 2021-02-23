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
    pub verbose: bool,
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
                    .help("Language to initialise the project for. Use `init list` to list supported languages."),
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
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Print extra information as the project is initialised.")
            )
            .get_matches();

    // Get the name of the current directory
    let cd_name = current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // If no name is specified or you specify ".", default to current directory name
    let project_name = if args.is_present("name") && args.value_of("name").unwrap() != "." {
        args.value_of("name").unwrap()
    } else {
        &cd_name
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
        verbose: args.is_present("verbose"),
    };

    config.info("parsed arguments and created config");

    // Ensure that the language is valid or list supported languages
    let directory = parse::get_directory();
    if config.language != "list"
        && (!directory.contains(&config.language)
            || !config.language.chars().all(char::is_alphabetic))
    {
        config.err("language not supported, try `init list` to list supported languages");
        return;
    } else if config.language == "list" {
        println!("init currently supports the following languages:\n");
        for language in directory.dirs() {
            println!("  - {}", language.path);
        }
        return;
    }

    config.info("validated language choice");

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
        if let Some(extras) = init_config.extras {
            let max = extras
                .iter()
                .max_by(|x, y| x.name.len().cmp(&y.name.len()))
                .unwrap()
                .name
                .len();
            for extra in extras {
                println!(
                    "  - {:width$}{}",
                    extra.name,
                    extra.description,
                    width = max + 8
                );
            }
        } else {
            println!("  - none");
        }

        return;
    }

    config.info("starting initialisation");

    init(&config, directory);
}

impl ProjectConfig {
    /// Runs `println!()` only if verbose is enabled
    fn info(&self, string: &str) {
        if self.verbose {
            println!("info:    {}", string);
        }
    }

    /// Runs `println!()`
    fn err(&self, string: &str) {
        println!("error:   {}", string);
    }
}
