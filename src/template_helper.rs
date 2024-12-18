use std::{error::Error, fs, path::Path};

use handlebars::Handlebars;
use serde_json::json;

use crate::runtime::Runtime;
use include_dir::{include_dir, Dir};

pub fn load_templates(
    runtime: Runtime,
    context: &str,
) -> Result<Handlebars<'static>, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    static TEMPLATES_DIR: Dir = include_dir!("templates"); // needed to embed templates dir within executable

    let template_dir_name = format!("{}_templates", context);
    let runtime_path = format!("{}/{}", template_dir_name, runtime.runtime_to_lowercase());

    let runtime_dir = TEMPLATES_DIR
        .get_dir(&runtime_path)
        .ok_or("Template directory not found")?;

    for file in runtime_dir.files() {
        let template_name = file
            .path()
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("Invalid template filename")?;

        let template_content = file.contents_utf8().ok_or("Invalid template content")?;

        handlebars.register_template_string(template_name, template_content)?;
    }

    Ok(handlebars)
}

pub fn create_project_files(
    name: &String,
    runtime: Runtime,
    context: &str,
    output_directory: &str,
) -> Result<(), Box<dyn Error>> {
    let handlebars = load_templates(runtime, context)?;
    let data = json!({
        "project_name": name,
        "runtime": runtime.to_string(),
    });

    let project_dir = Path::new(output_directory);
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }

    static TEMPLATES_DIR: Dir = include_dir!("templates");

    let template_dir_name = format!("{}_templates", context);
    let runtime_path = format!("{}/{}", template_dir_name, runtime.runtime_to_lowercase());

    let runtime_dir = TEMPLATES_DIR
        .get_dir(&runtime_path)
        .ok_or("Template directory not found")?;

    for file in runtime_dir.files() {
        let template_name = file
            .path()
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("Invalid template filename")?;

        let output_filename = file
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid output filename")?;

        let content = handlebars.render(template_name, &data)?;
        let output_path = project_dir.join(output_filename);
        fs::write(output_path, content)?;
    }

    Ok(())
}
