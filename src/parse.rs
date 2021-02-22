use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct InitFile {
    pub language: String,
    pub files: Vec<String>,
    #[serde(rename = "filesContainingExtras")]
    pub files_containing_extras: Vec<String>,
    pub extras: Vec<Extra>,
}

#[derive(Serialize, Deserialize)]
pub struct Extra {
    pub name: String,
    pub files: Vec<String>,
}

pub fn get_directory() -> Dir<'static> {
    return include_dir!("./data");
}

pub fn parse_init_file(file: &str) -> Result<InitFile> {
    return serde_json::from_str(file);
}

pub fn replace_placeholders(
    string: &str,
    project_name: &str,
    author: &str,
    description: &str,
) -> String {
    let project_name_lower = project_name.to_ascii_lowercase().replace(" ", "_");
    return string
        .replace("{{projectName}}", project_name)
        .replace("{{projectNameLower}}", &project_name_lower)
        .replace("{{author}}", author)
        .replace("{{projectDescription}}", description);
}
