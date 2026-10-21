provider "aws" {
  region = var.aws_region

  default_tags {
    tags = local.tags
  }
}


data "aws_eks_cluster_auth" "this" {
  name = module.eks.cluster_name
}
