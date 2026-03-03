variable "aws_region" {
  type = string
}

variable "env" {
  type        = string
  description = "The environment to deploy"
}

variable "account_id" {
  type        = string
  description = "AWS Account ID"
}

variable "name_prefix" {
  type        = string
  description = "The name prefix"
}

variable "pagerduty_service" {
  type = string
}

variable "datadog_api_key" {
  type = string
}

variable "datadog_app_key" {
  type = string
}
