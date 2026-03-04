module "obs" {
  depends_on = [
    module.eks
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

  env         = var.env
  account_id  = data.aws_caller_identity.current.account_id
  name_prefix = random_pet.client_prefix.id

  aws_region = module.net.aws_region

  k8s_cluster_name      = module.eks.cluster_name
  k8s_cluster_endpoint  = module.eks.cluster_endpoint
  k8s_cluster_version   = module.eks.cluster_version
  k8s_oidc_provider_arn = module.eks.oidc_provider_arn

  datadog_api_key = var.datadog_api_key
  datadog_app_key = var.datadog_app_key

  pagerduty_service = var.pagerduty_service
}
