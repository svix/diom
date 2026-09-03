module "diom" {
  providers = {
    kubernetes = kubernetes
    helm       = helm
  }
  source        = "./modules/diom"
  env           = var.env
  name_prefix   = local.name_prefix
  app_namespace = "diom-db"
}
