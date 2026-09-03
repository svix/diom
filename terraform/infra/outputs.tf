# network

output "env" {
  value = module.net.env
}

output "aws_region" {
  value = module.net.aws_region
}

output "name_prefix" {
  value = module.net.name_prefix
}

output "tags" {
  value = local.tags
}

output "vpc_id" {
  value = module.net.vpc_id
}

output "vpc_cidr" {
  value = module.net.vpc_cidr
}

output "public_subnet_ids" {
  value = module.net.public_subnet_ids
}

output "private_subnet_ids" {
  value = module.net.private_subnet_ids
}

output "database_subnet_ids" {
  value = module.net.database_subnet_ids
}

output "eks_subnet_ids" {
  value = module.net.eks_subnet_ids
}

output "dns_zone_arns" {
  value = module.net.dns_zone_arns
}

output "dns_zone_name" {
  value = module.net.dns_zone_name
}


# eks
output "k8s_endpoint" {
  value = module.eks.cluster_endpoint
}

output "k8s_cluster_name" {
  value = module.eks.cluster_name
}

output "k8s_cluster_version" {
  value = module.eks.cluster_version
}

output "k8s_cluster_certificate_authority_data" {
  value     = module.eks.cluster_certificate_authority_data
  sensitive = true
}

output "k8s_oidc_issuer" {
  value = module.eks.oidc_provider_url
}

output "k8s_oidc_provider_arn" {
  value = module.eks.oidc_provider_arn
}

output "diom_namespace" {
  value = var.diom_namespace
}

# output "tags" {
#   value = local.tags
# }

# output "dns_zone_arns" {
#   value = module.diom_aws.dns_zone_arns
# }

# output "svix_namespace" {
#   value = module.diom_aws.diom_namespace
# }

# output "domain_admin_email" {
#   value = var.domain_admin_email
# }

# output "api_domain_cert" {
#   value = module.diom_aws.api_domain_cert
# }

# output "app_domain_cert" {
#   value = module.diom_aws.app_domain_cert
# }

# output "frontend_domain_cert" {
#   value = module.diom_aws.frontend_domain_cert
# }

# output "ghcr_repo" {
#   value = var.ghcr_repo
# }

# output "ghcr_repo_username" {
#   value = var.ghcr_repo_username
# }

# output "ghcr_repo_secret" {
#   sensitive = true
#   value     = var.ghcr_repo_secret
# }

# output "ghcr_email" {
#   value = var.ghcr_email
# }

# output "api_endpoint" {
#   value = local.api_endpoint
# }

# output "static_endpoint" {
#   value = local.static_endpoint
# }

# output "frontend_endpoint" {
#   value = local.frontend_endpoint
# }

# output "pagerduty_service" {
#   value = var.pagerduty_service
# }

# output "pagerduty_notify_only_service" {
#   value = var.pagerduty_notify_only_service
# }

# output "alb_log_bucket" {
#   value = module.diom_aws.alb_log_bucket
# }
