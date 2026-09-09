module "obs" {
  depends_on = [
    module.tailscale # for prometheus
  ]

  source = "./modules/observability"
  providers = {
    aws        = aws,
    helm       = helm,
    kubectl    = kubectl,
    kubernetes = kubernetes,
    local      = local,
    tls        = tls,
    random     = random,
    # datadog = datadog
  }

  env        = local.env
  account_id = local.account_id

  aws_region  = local.aws_region
  name_prefix = local.name_prefix

  k8s_cluster_name      = local.k8s_cluster_name
  k8s_cluster_endpoint  = local.k8s_cluster_endpoint
  k8s_cluster_version   = local.k8s_cluster_version
  k8s_oidc_provider_arn = local.k8s_oidc_provider_arn

  # prometheus
  storage_class_name    = kubernetes_storage_class_v1.ebs_gp3.metadata[0].name
  tailscale_tsnet       = var.tailscale_tsnet
  pagerduty_routing_key = var.pagerduty_routing_key

  datadog_api_key = var.datadog_api_key
  datadog_app_key = var.datadog_app_key

  pagerduty_service = var.pagerduty_service
}
