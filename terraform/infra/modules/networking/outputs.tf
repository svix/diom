# network

output "aws_region" {
  value = var.aws_region
}

output "vpc_id" {
  value = module.vpc.vpc_id
}

output "vpc_cidr" {
  value = module.vpc.vpc_cidr_block
}

output "public_subnet_ids" {
  value = module.vpc.public_subnets
}

output "private_subnet_ids" {
  value = module.vpc.private_subnets
}

output "database_subnet_ids" {
  value = module.vpc.database_subnets
}

output "intra_subnet_ids" {
  value = module.vpc.intra_subnets
}

output "eks_subnet_ids" {
  value = local.eks_subnet_create ? aws_subnet.eks_system.*.id : []
}

output "dns_zone_arns" {
  value = local.zone_arns
}

output "dns_zone_name" {
  value = aws_route53_zone.ep_zone.name
}
