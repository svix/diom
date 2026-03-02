locals {
  name = "${var.name_prefix}-k8s"

  azs = ["${var.aws_region}a", "${var.aws_region}b"]

  subnet_cidrs = cidrsubnets(var.vpc_cidr, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)

  # eks_oidc_region_code = split(".", replace(module.eks.cluster_oidc_issuer_url, "https://", ""))[2]
  # eks_oidc_id          = split("/", replace(module.eks.cluster_oidc_issuer_url, "https://", ""))[2]

  diom_namespace = "svix-diom"

  zone_id   = try(data.aws_route53_zone.ep_zone[0].zone_id, aws_route53_zone.ep_zone[0].zone_id)
  zone_arns = [try(data.aws_route53_zone.ep_zone[0].arn, aws_route53_zone.ep_zone[0].arn)]

  api_domain       = "${var.api_domain_prefix}.${trimsuffix(var.customer_domain, ".")}"
  frontend_domain  = "${var.frontend_domain_prefix}.${trimsuffix(var.customer_domain, ".")}"
  frontend2_domain = "frontend.${trimsuffix(var.customer_domain, ".")}"
  cert_domain      = [local.api_domain, local.frontend_domain, local.frontend2_domain]

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

  dd_integration_policy = "${var.env}DatadogAWSIntegrationPolicy"
  dd_integration_role   = "${var.env}DatadogAWSIntegrationRole"

  # network eks
  eks_subnet_create  = true
  eks_subnet_len     = 2
  eks_subnet_suffix  = "eks"
  single_nat_gateway = true
  eks_cidr_index     = [8, 9] # until index 7, the subnets are created by vpc module
  nat_gateway_count  = local.single_nat_gateway ? 1 : length(local.azs)

  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))
}
