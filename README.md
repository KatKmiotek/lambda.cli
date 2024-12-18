# templates helper

## Installation

Steps:
1. **Step for Mac**: Run `curl -fsSL https://raw.githubusercontent.com/KatKmiotek/templates-helper/main/install.sh | sh`
2. **Verify**: Verify installation by running `template -V`
3. **Help**: To view all available commands run `template --help`

## Usage
### Lambda

To create lambda template run:
```sh
template lambda
```
provide:
- name for your project
- select language ( TypeScript, Dotnet, Python)
```shell
✔ Project name · my-project
? Select runtime ›
❯ TypeScript
  Dotnet
  Python
```
- select if Terraform file for new lambda needs to be created (output directory ./terraform/)
```sh
✔ Project name · my-project
✔ Select runtime · TypeScript
? Would you like to add terraform module? ›
❯ Yes
  No
  ```

### Terraform

To create terraform module template run:
```sh
template terraform
```
provide:
- name for your project that is also output directory path
```shell
? Project name (./terraform) ›
```
```sh
✔ Project name · ./terraform/sub-module/my-module
Terraform module ./terraform/sub-module/my-module has been created
```

## Local development

Tutorials:
1. **[Running project locally](#tutorial-1-local-setup)**
2. **[Installing pre-commit hooks](#tutorial-2-installing-pre-commit-hooks)**

---

### Tutorial 1: Running project locally

#### Objective
By following those steps you will setup your machine to run CLI Docs tool

Steps:
1. **Step 1**: Mac - Install rustup via [instruction](https://doc.rust-lang.org/cargo/getting-started/) from The Rust book
2. **Step 2**: Windows - Install rustup via [instruction](https://doc.rust-lang.org/cargo/getting-started/installation.html) from The Rust book
3. **Step 3**: Confirm `cargo -V` outputs version 1.82.0 or higher
4. **Step 4**: Run `cargo run` - this will build project

### Tutorial 2: Installing pre-commit hooks

#### Objective
Execute static code analysis locally

Steps:
1. **Step 1**: Install [pre-commit]()
2. **Step 2**: Run `pre-commit install` to add hook to .git/hooks/pre-commit - from now on git commit event staged files will be checked
3. **Step 3**: To run pre-commit on all files `pre-commit run --all-files`
