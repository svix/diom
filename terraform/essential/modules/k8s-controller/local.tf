locals {
  is_prod_env = !(startswith(var.env, "staging") || startswith(var.env, "dev"))

  sa_alb_ctr_name = "${var.name_prefix}-alb-ctr"
  sa_ext_dns_name = "${var.name_prefix}-ext-dns"

  ns_external_dns = "external-dns"
  ns_alb_ctr      = "ingress-controller"

  # zone_id   = try(data.aws_route53_zone.ep_zone[0].zone_id, aws_route53_zone.ep_zone[0].zone_id)
  # zone_arns = [try(data.aws_route53_zone.ep_zone[0].arn, aws_route53_zone.ep_zone[0].arn)]

  # api_domain       = "${var.api_domain_prefix}.${trimsuffix(var.customer_domain, ".")}"
  # frontend_domain  = "${var.frontend_domain_prefix}.${trimsuffix(var.customer_domain, ".")}"
  # frontend2_domain = "frontend.${trimsuffix(var.customer_domain, ".")}"
  # cert_domain      = [local.api_domain, local.frontend_domain, local.frontend2_domain]

}
