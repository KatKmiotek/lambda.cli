terraform {
  required_version = ">= 1.8.5"

  backend "s3" {
    region  = "eu-west-1"
    encrypt = true
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.81.0"
    }
  }
}

provider "aws" {
  default_tags {
    tags = local.default_tags
  }
}
