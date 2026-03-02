
# network

output "vpc_id" {
  value = module.vpc.vpc_id
}

output "vpc_cidr" {
  value = module.vpc.vpc_cidr_block
}

output "public_subnets" {
  value = module.vpc.public_subnets
}

output "private_subnets" {
  value = module.vpc.private_subnets
}

output "database_subnets" {
  value = module.vpc.database_subnets
}

output "elasticache_subnets" {
  value = module.vpc.elasticache_subnets
}

# eks
output "k8s_endpoint" {
  value = module.eks.cluster_endpoint
}

output "k8s_cluster_id" {
  value = module.eks.cluster_id
}

output "k8s_cluster_certificate_authority_data" {
  value = module.eks.cluster_certificate_authority_data
}

output "k8s_cluster_name" {
  value = module.eks.cluster_name
}

output "k8s_oidc_issuer" {
  value = module.eks.cluster_oidc_issuer_url
}

output "k8s_oidc_provider_arn" {
  value = module.eks.oidc_provider_arn
}

output "route53_zone" {
  value = var.use_existing_route53_zone ? data.aws_route53_zone.ep_zone[0].zone_id : aws_route53_zone.ep_zone[0].zone_id
}

output "route53_zone_name_servers" {
  value = var.use_existing_route53_zone ? data.aws_route53_zone.ep_zone[0].zone_id : aws_route53_zone.ep_zone[0].name_servers
}

output "dns_zone_arns" {
  value = local.zone_arns
}

output "diom_namespace" {
  value = local.diom_namespace
}

output "api_domain_cert" {
  value = module.alb_acm_cert[0].acm_certificate_arn
}

output "app_domain_cert" {
  value = module.alb_acm_cert[1].acm_certificate_arn
}

output "frontend_domain_cert" {
  value = module.alb_acm_cert[2].acm_certificate_arn
}

output "alb_log_bucket" {
  value = aws_s3_bucket.alb_log_bucket.bucket
}

output "eks_subnets" {
  value = [aws_subnet.eks_system[*].id]
}
