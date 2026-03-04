module "edge" {
  depends_on = [
    module.eks
  ]

  source = "./modules/edge"
  providers = {
    aws        = aws,
    kubernetes = kubernetes,
    helm       = helm,
    kubectl    = kubectl,
    local      = local,
    tls        = tls,
    random     = random,
    # datadog = datadog
  }

  env              = var.env
  account_id       = data.aws_caller_identity.current.account_id
  name_prefix      = random_pet.client_prefix.id
  diom_namespace = var.diom_namespace

  aws_region    = module.net.aws_region
  vpc_id        = module.net.vpc_id
  dns_zone_arns = module.net.dns_zone_arns
  dns_zone_name = module.net.dns_zone_name

  k8s_cluster_name      = module.eks.cluster_name
  k8s_cluster_endpoint  = module.eks.cluster_endpoint
  k8s_cluster_version   = module.eks.cluster_version
  k8s_oidc_provider_arn = module.eks.oidc_provider_arn
}
