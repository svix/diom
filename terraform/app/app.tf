module "coyote" {
  providers = {
    kubernetes = kubernetes
    helm       = helm
  }
  source        = "./modules/coyote"
  env           = var.env
  name_prefix   = local.name_prefix
  app_namespace = "coyote-db"
}
