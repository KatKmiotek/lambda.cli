#[derive(Debug, Clone, Copy)]
pub enum Runtime {
    TypeScript,
    Dotnet,
    Python,
    Terraform,
}

impl Runtime {
    pub fn runtime_to_lowercase(&self) -> String {
        match self {
            Runtime::TypeScript => "typescript",
            Runtime::Dotnet => "dotnet",
            Runtime::Python => "python",
            Runtime::Terraform => "terraform",
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
            Runtime::Terraform => write!(f, "Terraform"),
        }
    }
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
}
