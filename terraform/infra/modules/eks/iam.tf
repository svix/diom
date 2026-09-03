
module "aws_vpc_cni_ipv4_pod_identity" {
  source  = "terraform-aws-modules/eks-pod-identity/aws"
  version = "~> 2.7"

  name            = substr("${var.name_prefix}-aws-vpc-cni-ipv4", 0, 35)
  use_name_prefix = true

  attach_aws_vpc_cni_policy = true
  aws_vpc_cni_enable_ipv4   = true

  tags = var.tags
}

module "ebs_csi_irsa" {
  source          = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts"
  version         = "~> 6.4"
  name            = substr("${local.eks_cluster_name}-ebs-csi-role", 0, 35)
  use_name_prefix = true

  attach_ebs_csi_policy = true

  oidc_providers = {
    main = {
      provider_arn               = module.eks.oidc_provider_arn
      namespace_service_accounts = ["kube-system:ebs-csi-controller-sa"]
    }
  }
}

module "eks_sa_role_ext_dns" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts"
  version = "~> 6.4"

  name            = substr(local.sa_ext_dns_name, 0, 35)
  use_name_prefix = true

  attach_external_dns_policy    = true
  external_dns_hosted_zone_arns = var.dns_zone_arns

  oidc_providers = {
    ex = {
      provider_arn               = module.eks.oidc_provider_arn
      namespace_service_accounts = ["${local.ns_external_dns}:${local.sa_ext_dns_name}"]
    }
  }
}
