variable "aws_region" {
  type = string
}

variable "env" {
  type        = string
  description = "The environment to deploy"
}

variable "vpc_id" {
  type        = string
  description = "VPC ID of the aws network"
}

variable "account_id" {
  type        = string
  description = "AWS Account ID"
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

variable "name_prefix" {
  type        = string
  description = "The name prefix"
}

variable "dns_zone_arns" {
  type        = list(string)
  description = "Route53 zone arns for external dns to manage"
}

variable "diom_namespace" {
  type        = string
  description = "Diom namespace"
}

variable "dns_zone_name" {
  type        = string
  description = "DNS Zone name"
}
