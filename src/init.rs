use crate::{parse::InitConfig, ProjectConfig};
use fancy_regex::Regex;
use std::{
    fs::{create_dir_all, File},
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
    }

    let init_file_str = dir
        .get_file(format!("{}/init.json", config.language))
        .unwrap()
        .contents_utf8()
        .unwrap();

    let init_config: InitConfig = crate::parse::parse_init_file(init_file_str).unwrap();
    create_files(&init_config.files, &dir, config, &init_config);

    for extra in &init_config.extras {
        if config.extras.contains(&extra.name) {
            create_files(&extra.files, &dir, config, &init_config);
        }
    }
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
        let mut file_obj = File::create(file_path_obj).unwrap();
        let mut file_contents = dir
            .get_file(format!("{}/{}", config.language, file))
            .unwrap()
            .contents_utf8()
            .unwrap()
            .replace("\r\n", "\n"); // convert CRLF to LF because regex

        if init_config.files_containing_extras.contains(file) {
            let extra_regex =
                Regex::new(r#"( *\t*)#!startExtra ".*?"\n[\s\S]*?#!endExtra\n?"#).unwrap();
            let extra_name_regex = Regex::new(r#"(?<=#!startExtra ").*(?=")"#).unwrap();

            while extra_regex.is_match(&file_contents).unwrap() {
                let extra_full = extra_regex.find(&file_contents).unwrap().unwrap().as_str();
                let extra_name = extra_name_regex.find(extra_full).unwrap().unwrap().as_str();
                if config.extras.contains(&String::from(extra_name)) {
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
