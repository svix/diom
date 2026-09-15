variable "env" {
  type = string
}

variable "tailscale_client_id" {
  type = string
}

variable "tailscale_client_secret" {
  type = string
}

variable "chart_version" {
  type    = string
  default = "1.92.5"
}
