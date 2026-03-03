variable "aws_region" {
  type = string
}

variable "env" {
  type        = string
  description = "The environment to deploy"
}

variable "vpc_id" {
  type        = string
  description = "The VPC ID"
}

variable "private_subnet_ids" {
  type        = list(string)
  description = "The VPC private subnet IDs"
}

variable "database_subnet_ids" {
  type        = list(string)
  description = "The VPC database subnet IDs"
}

variable "control_plane_subnet_ids" {
  type        = list(string)
  description = "The control plane subnet IDs"
}

variable "account_id" {
  type        = string
  description = "AWS Account ID"
}

variable "elb_service_account_arn" {
  type        = string
  description = "ELB Service Account ARN"
}

variable "name_prefix" {
  type        = string
  description = "The name prefix"
}

variable "k8s_version" {
  type        = string
  description = "The version of Kubernetes to deploy"
}

variable "admin_users" {
  type        = list(string)
  description = "The list of admin users for the kubernetes cluster"
}

variable "admin_roles" {
  type        = list(string)
  description = "The list of admin roles for the kubernetes cluster"
}

variable "tags" {
  type = map(string)
}

variable "system_instance_types" {
  type = list(string)
}

variable "system_min_node_count" {
  type = number
}

variable "system_max_node_count" {
  type = number
}

variable "system_desired_node_count" {
  type = number
}

variable "system_ami_type" {
  type = string
}

variable "app_instance_types" {
  type = list(string)
}

variable "app_min_node_count" {
  type = number
}

variable "app_max_node_count" {
  type = number
}

variable "app_desired_node_count" {
  type = number
}

variable "app_ami_type" {
  type = string
}

variable "db_instance_types" {
  type = list(string)
}

variable "db_min_node_count" {
  type = number
}

variable "db_max_node_count" {
  type = number
}

variable "db_desired_node_count" {
  type = number
}

variable "db_ami_type" {
  type = string
}

variable "pagerduty_service" {
  type = string
}

variable "alb_log_expiration_days" {
  type = string
}

variable "eks_subnet_ids" {
  type = list(string)
}
