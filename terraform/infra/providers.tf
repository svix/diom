provider "aws" {
  region = var.aws_region

  default_tags {
    tags = local.tags
  }
}

# provider "datadog" {
#   api_key = var.datadog_api_key
#   app_key = var.datadog_app_key
# }
