module "obs" {
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
  aws_region  = data.aws_region.current.region
  account_id  = data.aws_caller_identity.current.account_id
  name_prefix = random_pet.client_prefix.id

  cluster_name      = module.eks.cluster_name
  cluster_endpoint  = module.eks.cluster_endpoint
  cluster_version   = module.eks.cluster_version
  oidc_provider_arn = module.eks.oidc_provider_arn

  datadog_api_key = var.datadog_api_key
  datadog_app_key = var.datadog_app_key

  pagerduty_service = var.pagerduty_service
}
