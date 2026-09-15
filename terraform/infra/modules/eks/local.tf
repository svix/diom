locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))

  eks_cluster_name = "${var.name_prefix}-eks"

  diom_namespace     = "svix-diom"
  sa_ext_dns_name      = "external-dns"
  ns_external_dns      = "external-dns"
  sa_cert_manager_name = "cert-manager"
  ns_cert_manager      = "cert-manager"


  admin_roles = {
    for k, v in var.admin_roles :
    "admin-roles-${k}" => {
      kubernetes_groups = [],
      principal_arn     = "${v}",
      policy_associations = {
        admin = {
          policy_arn = "arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy"
          access_scope = {
            type = "cluster"
          }
        }
      }
    }
  }

  admin_users = {
    for k, v in var.admin_users :
    "admin-users-${k}" => {
      kubernetes_groups = [],
      principal_arn     = "${v}",
      policy_associations = {
        admin = {
          policy_arn = "arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy"
          access_scope = {
            type = "cluster"
          }
        }
      }
    }
  }
}
