# no point in creating these addons through eks module's addons attribute.
# `before_compute` property does not create a correct dependency graph
# to create these addons before the nodegroups

data "aws_eks_addon_version" "eks_pod_identity_agent" {

  region = var.aws_region

  addon_name         = "eks-pod-identity-agent"
  kubernetes_version = module.eks.cluster_version
  most_recent        = true
}

data "aws_eks_addon_version" "vpc_cni" {

  region = var.aws_region

  addon_name         = "vpc-cni"
  kubernetes_version = module.eks.cluster_version
  most_recent        = true
}

resource "aws_eks_addon" "eks_pod_identity_agent" {
  cluster_name                = module.eks.cluster_name
  addon_name                  = "eks-pod-identity-agent"
  addon_version               = data.aws_eks_addon_version.eks_pod_identity_agent.version
  resolve_conflicts_on_create = "OVERWRITE"
  resolve_conflicts_on_update = "OVERWRITE"

}

resource "aws_eks_addon" "vpc_cni" {
  cluster_name                = module.eks.cluster_name
  addon_name                  = "vpc-cni"
  addon_version               = data.aws_eks_addon_version.vpc_cni.version
  resolve_conflicts_on_create = "OVERWRITE"
  resolve_conflicts_on_update = "OVERWRITE"

  depends_on = [aws_eks_addon.eks_pod_identity_agent]

  pod_identity_association {
    role_arn        = module.aws_vpc_cni_ipv4_pod_identity.iam_role_arn
    service_account = "aws-node"
  }
}
