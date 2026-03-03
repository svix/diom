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

variable "cluster_name" {
  type        = string
  description = "The EKS cluster name"
}

variable "cluster_endpoint" {
  type        = string
  description = "The EKS Cluster endpoint"
}

variable "cluster_version" {
  type        = string
  description = "The EKS Cluster version"
}

variable "oidc_provider_arn" {
  type        = string
  description = "The EKS OIDC provider ARN"
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
