use crate::{parse::InitConfig, ProjectConfig};
use fancy_regex::Regex;
use std::{
    fs::{create_dir_all, remove_file, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

pub fn init(config: &ProjectConfig, dir: include_dir::Dir) {
    if config.git {
        Command::new("git")
            .arg("init")
            .arg(&config.folder_name)
            .output()
            .expect("Please install git or pass the `--no-git` command.");
        config.info("initialised git repo");
    }

    let init_file_str = dir
        .get_file(format!("{}/init.json", config.language))
        .unwrap()
        .contents_utf8()
        .unwrap();
    config.info("read language init file");

    let init_config: InitConfig = crate::parse::parse_init_file(init_file_str).unwrap();

    // Copy base boilerplate files
    if let Some(files) = &init_config.files {
        create_files(files, &dir, config, &init_config);
        config.info("copied base boilerplate files");
    }

    // Run base commands
    if let Some(commands) = &init_config.commands {
        run_commands(commands, &config);
        config.info("run specified init commands");
    }

    // Copy extra files
    if let Some(extras) = &init_config.extras {
        for extra in extras {
            if config.extras.contains(&extra.name) {
                if let Some(extra_files) = &extra.files {
                    create_files(extra_files, &dir, config, &init_config);
                }
                if let Some(excluded_files) = &extra.excluded_files {
                    remove_files(excluded_files, config);
                }
                config.info(&format!("copied files for extra {}", extra.name));
            }
        }
    }

    config.info("completed initialistion");
}

fn create_files(
    files: &[String],
    dir: &include_dir::Dir,
    config: &ProjectConfig,
    init_config: &InitConfig,
) {
    for file in files {
        let file_path = format!(
            "{}/{}",
            config.folder_name,
            crate::parse::replace_placeholders(&file, config),
        );
        let file_path_obj = Path::new(&file_path);
        let prefix = file_path_obj.parent().unwrap();
        create_dir_all(prefix).unwrap();
        let mut file_obj = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path_obj)
            .unwrap();
        let mut file_contents = dir
            .get_file(format!("{}/{}", config.language, file))
            .unwrap()
            .contents_utf8()
            .unwrap()
            .replace("\r\n", "\n"); // convert CRLF to LF because regex

        if init_config.files_containing_extras.is_some()
            && init_config
                .files_containing_extras
                .as_ref()
                .unwrap()
                .contains(file)
        {
            let extra_regex =
                Regex::new(r#"( *\t*)(#|//)!startExtra ".*?"\n[\s\S]*?(#|//)!endExtra\n?"#)
                    .unwrap();
            let extra_name_regex = Regex::new(r#"(?<=#!startExtra ").*(?=")"#).unwrap();
            let extra_name_regex_alt = Regex::new(r#"(?<=//!startExtra ").*(?=")"#).unwrap();

            while extra_regex.is_match(&file_contents).unwrap() {
                let extra_full = extra_regex.find(&file_contents).unwrap().unwrap().as_str();
                let extra_name = if extra_name_regex.is_match(extra_full).unwrap() {
                    extra_name_regex.find(extra_full)
                } else {
                    extra_name_regex_alt.find(extra_full)
                }
                .unwrap()
                .unwrap()
                .as_str();

                if config.extras.contains(&String::from(extra_name))
                    || (!config.extras.contains(&String::from(&extra_name[1..]))
                        && extra_name.chars().nth(0).unwrap() == '!')
                {
                    let mut split_extra: Vec<&str> = extra_full.split('\n').collect();
                    split_extra.remove(0);
                    split_extra.remove(split_extra.len() - 2);
                    let extra_contents = split_extra.join("\n");
                    file_contents = file_contents.replace(extra_full, &extra_contents);
                } else {
                    file_contents = file_contents.replace(extra_full, "");
                }
            }
        }

        file_obj
            .write_all(crate::parse::replace_placeholders(&file_contents, config).as_bytes())
            .unwrap();
    }
}

fn remove_files(files: &[String], config: &ProjectConfig) {
    for file in files {
        let file_path = format!(
            "{}/{}",
            config.folder_name,
            crate::parse::replace_placeholders(&file, config),
        );
        remove_file(file_path).unwrap_or(());
    }
}

fn run_commands(commands: &[String], config: &ProjectConfig) {
    for command in commands {
        let parsed_command = crate::parse::replace_placeholders(command, config);
        let command_parts: Vec<&str> = parsed_command.split(" ").collect();
        Command::new(command_parts[0])
            .args(&command_parts[1..])
            .output()
            .expect("Command failed to run");
    }
}
