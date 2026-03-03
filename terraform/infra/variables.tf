
variable "env" {
  type        = string
  description = "The environment to deploy"
}

variable "aws_region" {
  type    = string
  default = "us-east-2"
}

variable "k8s_version" {
  type        = string
  description = "The version of Kubernetes to deploy"
  default     = "1.35"
}

variable "vpc_cidr" {
  type        = string
  description = "The CIDR block for the VPC. Expects a 16 byte mask"
  default     = "10.0.0.0/16"
}

variable "admin_users" {
  type        = list(string)
  description = "The list of admin users for the kubernetes cluster"
  default     = []
}

variable "admin_roles" {
  type        = list(string)
  description = "The list of admin roles for the kubernetes cluster"
  default     = []
}

variable "api_domain_prefix" {
  type    = string
  default = "api"
}

variable "frontend_domain_prefix" {
  type    = string
  default = "frontend"
}

variable "zone_name" {
  type    = string
  default = "diom.svix.dev"
}

variable "use_existing_route53_zone" {
  type    = bool
  default = false
}

variable "domain_admin_email" {
  type    = string
  default = "ashay.chitnis@yahoo.com"
}

variable "pagerduty_service" {
  type    = string
  default = "dd-test"
}

variable "pagerduty_notify_only_service" {
  type    = string
  default = "AWSCloudwatchStaging-Notify"
}

variable "datadog_api_key" {
  type = string
}

variable "datadog_app_key" {
  type = string
}

variable "ghcr_repo" {
  type = string
}

variable "ghcr_repo_username" {
  type = string
}

variable "ghcr_repo_secret" {
  type      = string
  sensitive = true
}

variable "ghcr_email" {
  type = string
}

variable "system_instance_types" {
  type    = list(string)
  default = ["t3.medium"]
}

variable "system_min_node_count" {
  type    = number
  default = 2
}

variable "system_max_node_count" {
  type    = number
  default = 3
}

variable "system_ami_type" {
  type    = string
  default = "AL2023_x86_64_STANDARD" # [valid values](https://docs.aws.amazon.com/eks/latest/APIReference/API_Nodegroup.html#AmazonEKS-Type-Nodegroup-amiType)
}

variable "system_desired_node_count" {
  type    = number
  default = 2
}

variable "app_instance_types" {
  type    = list(string)
  default = ["t3.micro"]
}

variable "app_min_node_count" {
  type    = number
  default = 1
}

variable "app_max_node_count" {
  type    = number
  default = 3
}

variable "app_ami_type" {
  type    = string
  default = "AL2023_x86_64_STANDARD" # [valid values](https://docs.aws.amazon.com/eks/latest/APIReference/API_Nodegroup.html#AmazonEKS-Type-Nodegroup-amiType)
}

variable "app_desired_node_count" {
  type    = number
  default = 1
}

variable "db_instance_types" {
  type    = list(string)
  default = ["t3.medium"]
}

variable "db_min_node_count" {
  type    = number
  default = 1
}

variable "db_max_node_count" {
  type    = number
  default = 3
}

variable "db_ami_type" {
  type    = string
  default = "AL2023_x86_64_STANDARD" # [valid values](https://docs.aws.amazon.com/eks/latest/APIReference/API_Nodegroup.html#AmazonEKS-Type-Nodegroup-amiType)
}

variable "db_desired_node_count" {
  type    = number
  default = 1
}

variable "alb_log_expiration_days" {
  type    = number
  default = 31
}
