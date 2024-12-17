use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use handlebars::Handlebars;
use serde_json::json;
use std::{error::Error, fs, path::Path, process::Command};

#[derive(Parser)]
#[command(name = "template")]
#[clap(about = "Project template generator", long_about = None, version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        about = "Generates new lambda project (TS, .NET, Python) with optional terraform for it"
    )]
    Lambda {},
}

#[derive(Debug, Clone, Copy)]
enum Runtime {
    TypeScript,
    Dotnet,
    Python,
}

impl Runtime {
    fn runtime_to_lowercase(&self) -> String {
        match self {
            Runtime::TypeScript => "typescript",
            Runtime::Dotnet => "dotnet",
            Runtime::Python => "python",
        }
        .to_string()
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Runtime::TypeScript => write!(f, "TypeScript"),
            Runtime::Dotnet => write!(f, "Dotnet"),
            Runtime::Python => write!(f, "Python"),
        }
    }
}

fn load_templates(runtime: Runtime, context: &str) -> Result<Handlebars<'static>, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    let template_dir_name = format!("{}_templates", context);
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

fn create_dotnet_lambda(name: &str) -> Result<String, Box<dyn Error>> {
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
    Ok(name.to_string())
}

fn create_project_files(
    name: &String,
    runtime: Runtime,
    context: &str,
) -> Result<String, Box<dyn Error>> {
    let handlebars = load_templates(runtime, context).unwrap();
    let data = json!({
        "project_name": name,
        "runtime": runtime.to_string(),
    });

    let project_dir_name = match context {
        "terraform" => "terraform".to_string(),
        "lambda" => format!("src/{}", name),
        _ => todo!(),
    };
    let project_dir = Path::new(&project_dir_name);
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }

    let template_dir =
        Path::new(&format!("{}_templates", context)).join(runtime.runtime_to_lowercase());

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
    Ok(project_dir_name)
}

fn main() {
    Cli::parse();
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

    let yes_no = &["Yes", "No"];
    let need_terraform = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add terraform module?")
        .default(0)
        .items(&yes_no[..])
        .interact()
        .unwrap();

    let project_directory = match runtimes[runtime] {
        Runtime::Dotnet => create_dotnet_lambda(&name).unwrap(),
        Runtime::Python | Runtime::TypeScript => {
            create_project_files(&name, runtimes[runtime], "lambda").unwrap()
        }
    };

    run_post_creation_commands(&project_directory, runtimes[runtime]).unwrap();

    match need_terraform {
        0 => {
            create_project_files(&name, runtimes[runtime], "terraform").unwrap();
        }
        _ => println!("No terraform will be created"),
    }

    println!(
        "Your {} lambda in {:?} has been created",
        name, runtimes[runtime]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_conversions() {
        let runtime = Runtime::TypeScript;
        assert_eq!(runtime.runtime_to_lowercase(), "typescript");
        assert_eq!(runtime.to_string(), "TypeScript");

        let runtime = Runtime::Python;
        assert_eq!(runtime.runtime_to_lowercase(), "python");
        assert_eq!(runtime.to_string(), "Python");
    }

    // #[test]
    // fn test_error_cases() {
    //     let result = load_templates(Runtime::TypeScript, "nonexistent");
    //     assert!(result.is_err());

    //     let result =
    //         create_project_files(&String::from(""), Runtime::TypeScript, "invalid_context");
    //     assert!(result.is_err());
    // }
}
