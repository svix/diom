provider "aws" {
  region = local.aws_region

  default_tags {
    tags = local.tags
  }
}


data "aws_eks_cluster_auth" "this" {
  name = local.k8s_cluster_name
}

# ToDo Read from tfe remote (`coyote-infra-*` workspace)
provider "kubernetes" {
  host                   = local.k8s_cluster_endpoint
  cluster_ca_certificate = base64decode(local.k8s_cluster_certificate_authority_data)
  token                  = data.aws_eks_cluster_auth.this.token
}

provider "kubectl" {
  load_config_file       = "false"
  host                   = local.k8s_cluster_endpoint
  cluster_ca_certificate = base64decode(local.k8s_cluster_certificate_authority_data)
  token                  = data.aws_eks_cluster_auth.this.token
}

provider "helm" {
  kubernetes = {
    host                   = local.k8s_cluster_endpoint
    cluster_ca_certificate = base64decode(local.k8s_cluster_certificate_authority_data)
    token                  = data.aws_eks_cluster_auth.this.token
  }
}

provider "tfe" {
  token = var.tfe_token
}

# provider "datadog" {
#   api_key = var.datadog_api_key
#   app_key = var.datadog_app_key
# }
