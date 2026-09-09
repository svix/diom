locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))
}
