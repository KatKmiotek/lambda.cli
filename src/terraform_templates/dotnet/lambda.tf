module "lambda_function" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "{{project_name}}"
  description   = "Lambda function written in {{runtime}}"
  handler       = "{{project_name}}::{{project_name}}.Function::FunctionHandler"
  runtime       = "dotnet8"
  source_path = [{
    path = "../src"
    commands = [
      "dotnet restore",
      "dotnet publish -c Release -r linux-arm64 -o publish",
      "cd ./publish",
      ":zip"
    ]
  }]
  publish = true
  environment_variables = {
    ENV = "dev"
  }
  attach_policy_statements = true
  policy_statements = {
    cloud_watch = {
      effect    = "Allow",
      actions   = ["cloudwatch:PutMetricData"],
      resources = ["*"]
    }
  }
  tags = {
    Name = "{{project_name}}"
  }
}
