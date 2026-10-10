module "tailscale" {
  source = "./modules/tailscale"

  providers = {
    kubernetes = kubernetes
    helm       = helm
  }

  env = local.env

  tailscale_client_id     = var.tailscale_client_id
  tailscale_client_secret = var.tailscale_client_secret
}
