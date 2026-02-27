module "ic" {
  source = "./modules/k8s-controller"
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

  env              = local.env
  account_id       = local.account_id
  coyote_namespace = var.coyote_namespace

  aws_region    = local.aws_region
  vpc_id        = local.vpc_id
  name_prefix   = local.name_prefix
  dns_zone_arns = local.dns_zone_arns
  dns_zone_name = local.dns_zone_name

  k8s_cluster_name      = local.k8s_cluster_name
  k8s_cluster_endpoint  = local.k8s_cluster_endpoint
  k8s_cluster_version   = local.k8s_cluster_version
  k8s_oidc_provider_arn = local.k8s_oidc_provider_arn
}
