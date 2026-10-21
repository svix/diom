resource "helm_release" "tailscale" {
  create_namespace = true
  repository       = "https://pkgs.tailscale.com/helmcharts"
  version          = var.chart_version
  namespace        = "tailscale"
  chart            = "tailscale-operator"
  name             = "tailscale-operator"

  set_sensitive = [
    {
      name  = "oauth.clientId"
      value = var.tailscale_client_id
    },
    {
      name  = "oauth.clientSecret"
      value = var.tailscale_client_secret
    }
  ]

  set = [
    {
      name  = "operatorConfig.defaultTags"
      value = "tag:k8s-operator-${var.env}"
    },
    {
      name  = "proxyConfig.defaultTags"
      value = "tag:k8s-${var.env}"
    }
  ]
}
