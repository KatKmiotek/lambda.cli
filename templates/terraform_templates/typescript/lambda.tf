module "lambda_function" {
  source        = "terraform-aws-modules/lambda/aws"
  function_name = "{{project_name}}"
  description   = "Lambda function written in {{runtime}}"
  handler       = "index.handler"
  runtime       = "nodejs20.x"
  source_path = [{
    path = "../src"
    commands = [
      "npm ci",        # install dependencies
      "npm run build", # npx tsc to transpile
      "npm prune --production",
      ":zip" # zip all
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
