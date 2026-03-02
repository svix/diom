resource "random_pet" "client_prefix" {
  prefix    = "${local.name_prefix}-"
  length    = 1
  separator = ""
}
