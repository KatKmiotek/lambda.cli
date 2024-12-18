use std::error::Error;

use dialoguer::{theme::ColorfulTheme, Input};

use crate::{runtime::Runtime, template_helper::create_project_files};

pub fn create_terraform_project() -> Result<(), Box<dyn Error>> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("terraform".to_string())
        .interact_text()
        .unwrap();

    create_project_files(&name, Runtime::Terraform, "terraform_module", &name).unwrap();
    println!("Terraform module {} has been created", name);
    Ok(())
}
