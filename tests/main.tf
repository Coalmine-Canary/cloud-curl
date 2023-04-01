terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.60"
    }
  }
}

provider "aws" {
  region = "us-east-1"
}

module "elasticsearch" {
  source        = "./modules/elasticsearch"
  cluster_name  = "cloud-curl-testing"
}
