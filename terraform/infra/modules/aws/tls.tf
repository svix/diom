# module "acm_cert" {
#   for_each = toset([local.api_domain, local.frontend_domain])

#   source  = "terraform-aws-modules/acm/aws"
#   version = "~> 5.1.0"

#   domain_name = each.value
#   zone_id     = local.zone_id

#   validation_method = "DNS"

#   tags = {
#     Name = each.value
#   }
# }

module "alb_acm_cert" {
  depends_on = [
    aws_route53_record.customer_caa
  ]

  count = length(local.cert_domain)

  source  = "terraform-aws-modules/acm/aws"
  version = "~> 6.3"

  zone_id                   = local.zone_id
  domain_name               = "alt-${local.cert_domain[count.index]}"
  subject_alternative_names = [local.cert_domain[count.index]]

  validation_method = "DNS"

  tags = {
    Name = local.cert_domain[count.index]
  }
}

resource "aws_route53_record" "customer_caa" {
  zone_id = var.use_existing_route53_zone ? data.aws_route53_zone.ep_zone[0].zone_id : aws_route53_zone.ep_zone[0].zone_id
  name    = var.use_existing_route53_zone ? data.aws_route53_zone.ep_zone[0].name : aws_route53_zone.ep_zone[0].name
  type    = "CAA"
  ttl     = 300

  records = [
    "0 issue \"amazon.com\""
  ]
}
