
variable "env" {
  type = string
  # uses workspace env data
}

variable "tfe_token" {
  type      = string
  sensitive = true
}

variable "coyote_infra_workspace" {
  type    = string
  default = "coyote-infra-dev"
}
