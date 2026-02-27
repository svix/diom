data "tfe_outputs" "coyote_infra" {
  # for dev and staging environments, we seek the monorepo clickhouse eks endpoint details
  # for prod we use the coyote eks infra (the infra folder in this repo)
  organization = var.env == "dev" ? "svix-development" : "svix"
  workspace    = var.env == "dev" ? "svix-dev" : var.env == "staging" ? "svix-monorepo-staging" : var.coyote_infra_workspace
}

data "aws_caller_identity" "current" {}

locals {
  name_prefix = "${var.env}-coyote"

  aws_region                             = var.env == "dev" ? data.tfe_outputs.coyote_infra.values.coyote_eks_region : var.env == "staging" ? data.tfe_outputs.coyote_infra.values.usw2_coyote_eks_region : data.tfe_outputs.coyote_infra.values.aws_region
  k8s_cluster_endpoint                   = var.env == "dev" ? data.tfe_outputs.coyote_infra.values.coyote_eks_endpoint : var.env == "staging" ? data.tfe_outputs.coyote_infra.values.usw2_coyote_eks_endpoint : data.tfe_outputs.coyote_infra.values.k8s_endpoint
  k8s_cluster_certificate_authority_data = var.env == "dev" ? data.tfe_outputs.coyote_infra.values.coyote_eks_cluster_certificate_authority_data : var.env == "staging" ? data.tfe_outputs.coyote_infra.values.usw2_coyote_eks_cluster_certificate_authority_data : data.tfe_outputs.coyote_infra.values.k8s_cluster_certificate_authority_data
  k8s_cluster_name                       = var.env == "dev" ? data.tfe_outputs.coyote_infra.values.coyote_eks_name : var.env == "staging" ? data.tfe_outputs.coyote_infra.values.usw2_coyote_eks_name : data.tfe_outputs.coyote_infra.values.k8s_cluster_name
  tags = {
    "Managed-By" = "terraform"
  }
}
