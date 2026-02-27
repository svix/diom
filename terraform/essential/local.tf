data "tfe_outputs" "coyote_infra" {
  organization = "svix-development"
  workspace    = var.coyote_infra_workspace
}

data "aws_caller_identity" "current" {}

locals {
  account_id = data.aws_caller_identity.current.account_id

  env           = data.tfe_outputs.coyote_infra.values.env
  aws_region    = data.tfe_outputs.coyote_infra.values.aws_region
  name_prefix   = data.tfe_outputs.coyote_infra.values.name_prefix
  tags          = data.tfe_outputs.coyote_infra.values.tags
  vpc_id        = data.tfe_outputs.coyote_infra.values.vpc_id
  dns_zone_arns = data.tfe_outputs.coyote_infra.values.dns_zone_arns
  dns_zone_name = data.tfe_outputs.coyote_infra.values.dns_zone_name

  k8s_cluster_endpoint                   = data.tfe_outputs.coyote_infra.values.k8s_endpoint
  k8s_cluster_certificate_authority_data = data.tfe_outputs.coyote_infra.values.k8s_cluster_certificate_authority_data
  k8s_cluster_name                       = data.tfe_outputs.coyote_infra.values.k8s_cluster_name
  k8s_cluster_version                    = data.tfe_outputs.coyote_infra.values.k8s_cluster_version
  k8s_oidc_issuer                        = data.tfe_outputs.coyote_infra.values.k8s_oidc_issuer
  k8s_oidc_provider_arn                  = data.tfe_outputs.coyote_infra.values.k8s_oidc_provider_arn
}
