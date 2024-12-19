use std::{error::Error, fs, path::Path, process::Command};

use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{runtime::Runtime, template_helper::create_project_files};

pub fn create_dotnet_lambda(name: &str) -> Result<(), Box<dyn Error>> {
    println!("Installing .NET tools...");
    Command::new("dotnet")
        .arg("new")
        .arg("install")
        .arg("Amazon.Lambda.Templates")
        .status()
        .expect("Failed to install templates");

    println!("Creating .NET Lambda project...");
    Command::new("dotnet")
        .arg("new")
        .arg("lambda.EmptyFunction")
        .arg("--name")
        .arg(name)
        .status()
        .expect("Failed to create .NET Lambda project");

    println!("Creating .gitignore file...");
    let gitignore_content = r#"bin/
obj/
.vs/
*.user
*.userosscache
*.suo
.vscode/
.idea/
*.swp
*.*~
project.lock.json
.DS_Store
*.pyc
"#;

    let gitignore_path = Path::new(name).join(".gitignore");
    fs::write(gitignore_path, gitignore_content)?;

    Ok(())
}

fn run_post_creation_commands(project_name: &str, runtime: Runtime) -> Result<(), Box<dyn Error>> {
    if let Runtime::TypeScript = runtime {
        println!("Installing npm dependencies...");
        let status = Command::new("npm")
            .current_dir(project_name)
            .arg("install")
            .status()?;
        if !status.success() {
            return Err("npm install failed".into());
        }
    }

    Ok(())
}

pub fn create_lambda_project() -> Result<(), Box<dyn Error>> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    let runtimes = &[Runtime::TypeScript, Runtime::Dotnet, Runtime::Python];

    let runtime = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runtime")
        .default(0)
        .items(&runtimes[..])
        .interact()
        .unwrap();
    let output_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Where to save lambda project?")
        .default(format!("src/{}", name))
        .interact_text()
        .unwrap();

    let yes_no = &["Yes", "No"];
    let need_terraform = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add terraform module?")
        .default(0)
        .items(&yes_no[..])
        .interact()
        .unwrap();

    match runtimes[runtime] {
        Runtime::Dotnet => create_dotnet_lambda(&name).unwrap(),
        Runtime::Python | Runtime::TypeScript => {
            create_project_files(&name, runtimes[runtime], "lambda", &output_path).unwrap()
        }
        _ => todo!(),
    };

    run_post_creation_commands(&output_path, runtimes[runtime]).unwrap();

    if need_terraform == 0 {
        create_project_files(&name, runtimes[runtime], "terraform", "terraform").unwrap();
    }

    println!(
        "Your {} lambda in {:?} has been created",
        name, runtimes[runtime]
    );
    Ok(())
}
