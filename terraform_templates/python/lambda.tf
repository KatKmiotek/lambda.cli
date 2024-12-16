module "lambda_function" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "{{project_name}}"
  description   = "Lambda function written in {{runtime}}"
  handler       = "lambda_function.lambda_handler"
  runtime       = "python3.12"

  source_path = "../src/{{project_name}}"
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
