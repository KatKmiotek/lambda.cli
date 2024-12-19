use std::error::Error;

use dialoguer::Confirm;

use crate::{runtime::Runtime, template_helper::create_project_files};

pub fn create_pipeline_files() -> Result<(), Box<dyn Error>> {
    let name = String::from("pipeline");
    let acknowledge = Confirm::new()
        .with_prompt("Do you want to create files in .github/workflows")
        .interact()
        .unwrap();
    if acknowledge {
        create_project_files(&name, Runtime::GitHub, &name, ".github").unwrap();
    }
    Ok(())
}
