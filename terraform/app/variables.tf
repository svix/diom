
variable "env" {
  type = string
  # uses workspace env data
}

variable "tfe_token" {
  type      = string
  sensitive = true
}

variable "diom_infra_workspace" {
  type    = string
  default = "diom-infra-dev"
}
