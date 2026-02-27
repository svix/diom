
variable "db_desired_node_count" {
  type    = number
  default = 1
}

variable "alb_log_expiration_days" {
  type    = number
  default = 31
}

variable "coyote_namespace" {
  type    = string
  default = "coyote-db"
}

variable "tailscale_client_id" {
  type    = string
  default = "x"
}

variable "tailscale_client_secret" {
  type      = string
  default   = "x"
  sensitive = true
}

variable "coyote_infra_workspace" {
  type = string
}

variable "pagerduty_service" {
  type = string
}

variable "pagerduty_routing_key" {
  type    = string
  default = "ifkbrnwqw4lrg39t8tsrjhj9gvzx3ew5" #dummy
}

variable "datadog_app_key" {
  type = string
}

variable "datadog_api_key" {
  type = string
}

variable "tfe_token" {
  type      = string
  sensitive = true
}

variable "tailscale_tsnet" {
  type    = string
  default = "lykoi-carp.ts.net"
}
