locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))

  eks_cluster_name = "${var.name_prefix}-k8s"

  diom_namespace = "svix-diom"

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
