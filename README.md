# Lambda templates helper

## Installation

Steps:
1. **Step for Mac**: Run `curl -fsSL https://raw.githubusercontent.com/KatKmiotek/lambda.cli/main/install.sh | sh`
2. **Verify**: Verify installation by running `template -v`
3. **Execute**: Now, you can use `template` to run application
4. **Help**: To view all available commands run `template --help`

## Usage

To create lambda template run:
```sh
template lambda
```
provide:
- name for your project
- select language ( TypeScript, Dotnet, Python)
- select if Terraform file for new lambda needs to be created (output directory ./terraform/)
