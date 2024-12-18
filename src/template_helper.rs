use std::{error::Error, fs, path::Path};

use handlebars::Handlebars;
use serde_json::json;

use crate::runtime::Runtime;

pub fn load_templates(
    runtime: Runtime,
    context: &str,
) -> Result<Handlebars<'static>, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    let template_dir_name = format!("src/{}_templates", context);
    let template_dir = Path::new(&template_dir_name).join(runtime.runtime_to_lowercase());
    for entry in fs::read_dir(&template_dir).expect("Directory doesn't exist") {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let template_name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .ok_or("Invalid template filename")?;

            let template_content = fs::read_to_string(&path)?;

            handlebars.register_template_string(template_name, template_content)?;
        }
    }

    Ok(handlebars)
}

pub fn create_project_files(
    name: &String,
    runtime: Runtime,
    context: &str,
    output_directory: &str,
) -> Result<(), Box<dyn Error>> {
    let handlebars = load_templates(runtime, context).unwrap();
    let data = json!({
        "project_name": name,
        "runtime": runtime.to_string(),
    });

    let project_dir = Path::new(&output_directory);
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }

    let template_dir =
        Path::new(&format!("src/{}_templates", context)).join(runtime.runtime_to_lowercase());

    for entry in fs::read_dir(template_dir).expect("Directory doesn't exist") {
        let entry = entry?;
        let template_path = entry.path();

        if template_path.is_file() {
            let template_name = template_path
                .file_stem()
                .and_then(|n| n.to_str())
                .ok_or("Invalid template filename")?;

            let output_filename = template_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or("Invalid output filename")?;

            let content = handlebars.render(template_name, &data)?;

            let output_path = project_dir.join(output_filename);
            fs::write(output_path, content)?
        }
    }
    Ok(())
}
