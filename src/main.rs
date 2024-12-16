use std::{error::Error, fs, path::Path, process::Command};

use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use handlebars::Handlebars;
use serde_json::json;

#[derive(Parser)]
#[command(name = "template")]
#[command(about = "Project template generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Lambda {},
}

fn load_templates(runtime: &str, context: &str) -> Result<Handlebars<'static>, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    let template_dir_name = format!("{}_templates", context);
    let template_dir = Path::new(template_dir_name.as_str()).join(runtime.to_lowercase());
    for entry in fs::read_dir(&template_dir).unwrap() {
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

fn create_lambda_files(name: &str, runtime: &str) -> Result<String, Box<dyn Error>> {
    let handlebars = load_templates(runtime, "lambda")?;

    let data = json!({
        "project_name": name,
        "runtime": runtime,
    });
    let project_dir_name = format!("{}/src", name);

    let project_dir = Path::new(project_dir_name.as_str());
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }

    let template_dir = Path::new("lambda_templates").join(runtime.to_lowercase());

    for entry in fs::read_dir(template_dir).unwrap() {
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
            fs::write(output_path, content)?;
        }
    }

    Ok(project_dir_name)
}

fn run_post_creation_commands(project_name: &str, runtime: &str) -> Result<(), Box<dyn Error>> {
    if runtime == "TypeScript" {
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

fn create_terraform_files(name: &String, runtime: &str) -> Result<(), Box<dyn Error>> {
    let handlebars = load_templates(runtime, "terraform").unwrap();
    let data = json!({
        "project_name": name,
        "runtime": runtime,
    });
    let project_dir = Path::new("terraform");
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }

    let template_dir = Path::new("terraform_templates").join(runtime.to_lowercase());

    for entry in fs::read_dir(template_dir).unwrap() {
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
            fs::write(output_path, content)?;
        }
    }
    Ok(())
}
fn main() {
    Cli::parse();
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .interact_text()
        .unwrap();

    let runtimes = &["TypeScript", "Dotnet", "Python"];

    let runtime = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select runtime")
        .default(0)
        .items(&runtimes[..])
        .interact()
        .unwrap();

    let yes_no = &["Yes", "No"];
    let need_terraform = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add terraform module?")
        .default(0)
        .items(&yes_no[..])
        .interact()
        .unwrap();
    let project_directory = match runtimes[runtime] {
        "Dotnet" => {
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
                .arg(&name)
                .status()
                .expect("Failed to create .NET Lambda project");
            name.clone()
        }
        "Python" | "TypeScript" => create_lambda_files(&name, runtimes[runtime]).unwrap(),
        _ => todo!(),
    };
    run_post_creation_commands(&project_directory, runtimes[runtime]).unwrap();
    match need_terraform {
        0 => create_terraform_files(&name, runtimes[runtime]).unwrap(),
        1 => {
            println!("No terraform will be created")
        }
        _ => todo!(),
    }

    println!(
        "Your {} lambda in {} has been created",
        name, runtimes[runtime]
    );
}
