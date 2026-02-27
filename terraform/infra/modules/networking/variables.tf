variable "aws_region" {
  type = string
}

variable "env" {
  type        = string
  description = "The environment to deploy"
}

variable "name_prefix" {
  type        = string
  description = "The name prefix"
}

variable "vpc_cidr" {
  type        = string
  description = "The CIDR block for the VPC. Expects a 16 byte mask"
}

variable "frontend_domain_prefix" {
  type = string
}

variable "alb_log_expiration_days" {
  type = string
}

variable "api_domain_prefix" {
  type = string
}

variable "coyote_domain" {
  type = string
}

variable "use_existing_route53_zone" {
  type = bool
}

variable "tags" {
  type = map(string)
}
