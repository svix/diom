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

variable "k8s_cluster_name" {
  type        = string
  description = "The EKS cluster name"
}

variable "k8s_cluster_endpoint" {
  type        = string
  description = "The EKS Cluster endpoint"
}

variable "k8s_cluster_version" {
  type        = string
  description = "The EKS Cluster version"
}

variable "k8s_oidc_provider_arn" {
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

variable "storage_class_name" {
  type = string
}

variable "pagerduty_routing_key" {
  type = string
}

variable "tailscale_tsnet" {
  type = string
}
