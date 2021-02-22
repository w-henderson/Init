mod parse;

use parse::InitFile;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::Command,
};

fn main() {
    init("python", "PogChamp", "William Henderson", vec![]);
}

fn init(data_name: &str, project_name: &str, author: &str, extras: Vec<String>) {
    let dir = parse::get_directory();
    let init_file_str = dir
        .get_file(format!("{}/init.json", data_name))
        .unwrap()
        .contents_utf8()
        .unwrap();

    let init_file: InitFile = parse::parse_init_file(init_file_str).unwrap();
    create_files(&init_file.files, &dir, data_name, project_name, author, "");

    for extra in init_file.extras {
        if extras.contains(&extra.name) {
            create_files(&extra.files, &dir, data_name, project_name, author, "");
        }
    }

    println!("{}", init_file.language);
}

fn create_files(
    files: &Vec<String>,
    dir: &include_dir::Dir,
    data_name: &str,
    project_name: &str,
    author: &str,
    description: &str,
) {
    let project_name_no_spaces = project_name.replace(" ", "");

    for file in files {
        let file_path = format!(
            "{}/{}",
            &project_name_no_spaces,
            parse::replace_placeholders(&file, project_name, author, description),
        );
        let file_path_obj = Path::new(&file_path);
        let prefix = file_path_obj.parent().unwrap();
        create_dir_all(prefix).unwrap();
        let mut file_obj = File::create(file_path_obj).unwrap();
        let file_contents = dir
            .get_file(format!("{}/{}", data_name, file))
            .unwrap()
            .contents_utf8()
            .unwrap();
        file_obj
            .write(
                parse::replace_placeholders(file_contents, project_name, author, description)
                    .as_bytes(),
            )
            .unwrap();
    }
}
