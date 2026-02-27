locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))

  name = var.name_prefix

  azs = ["${var.aws_region}a", "${var.aws_region}b", "${var.aws_region}c"]

  subnet_cidrs = cidrsubnets(var.vpc_cidr, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)

  coyote_namespace = "svix-coyote"

  zone_id   = aws_route53_zone.ep_zone.zone_id
  zone_arns = [aws_route53_zone.ep_zone.arn]

  api_domain      = "${var.api_domain_prefix}.${trimsuffix(var.coyote_domain, ".")}"
  frontend_domain = "${var.frontend_domain_prefix}.${trimsuffix(var.coyote_domain, ".")}"
  cert_domain     = [local.api_domain, local.frontend_domain]

  # network eks
  eks_subnet_create  = true
  eks_subnet_len     = 3
  eks_subnet_suffix  = "eks"
  single_nat_gateway = true
  eks_cidr_index     = [9, 10, 11] # until index 8, the subnets are created by vpc module
  nat_gateway_count  = local.single_nat_gateway ? 1 : length(local.azs)

}
