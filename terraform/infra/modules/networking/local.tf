locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))

  name = "${var.name_prefix}-k8s"

  azs = ["${var.aws_region}a", "${var.aws_region}b", "${var.aws_region}c"]

  subnet_cidrs = cidrsubnets(var.vpc_cidr, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)

  diom_namespace = "svix-diom"

  zone_id   = try(data.aws_route53_zone.ep_zone[0].zone_id, aws_route53_zone.ep_zone[0].zone_id)
  zone_arns = [try(data.aws_route53_zone.ep_zone[0].arn, aws_route53_zone.ep_zone[0].arn)]

  api_domain      = "${var.api_domain_prefix}.${trimsuffix(var.diom_domain, ".")}"
  frontend_domain = "${var.frontend_domain_prefix}.${trimsuffix(var.diom_domain, ".")}"
  cert_domain     = [local.api_domain, local.frontend_domain]

  # network eks
  eks_subnet_create  = true
  eks_subnet_len     = 2
  eks_subnet_suffix  = "eks"
  single_nat_gateway = true
  eks_cidr_index     = [8, 9] # until index 7, the subnets are created by vpc module
  nat_gateway_count  = local.single_nat_gateway ? 1 : length(local.azs)

}
