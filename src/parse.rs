use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::ProjectConfig;

#[derive(Serialize, Deserialize)]
pub struct InitConfig {
    pub language: String,
    pub files: Option<Vec<String>>,
    pub commands: Option<Vec<String>>,
    #[serde(rename = "filesContainingExtras")]
    pub files_containing_extras: Option<Vec<String>>,
    pub extras: Option<Vec<Extra>>,
}

#[derive(Serialize, Deserialize)]
pub struct Extra {
    pub name: String,
    pub description: String,
    pub files: Option<Vec<String>>,
    #[serde(rename = "excludedFiles")]
    pub excluded_files: Option<Vec<String>>,
}

pub fn get_directory() -> Dir<'static> {
    include_dir!("./data")
}

pub fn parse_init_file(file: &str) -> Result<InitConfig> {
    serde_json::from_str(file)
}

pub fn replace_placeholders(string: &str, config: &ProjectConfig) -> String {
    let project_name_lower = config.name.to_ascii_lowercase().replace(" ", "_");
    string
        .replace("{{projectName}}", &config.name)
        .replace("{{projectNameLower}}", &project_name_lower)
        .replace("{{author}}", &config.author)
        .replace("{{projectDescription}}", &config.description)
        .replace("{{folderPath}}", &config.folder_name)
}
