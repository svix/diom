# configures ACM and externalDNS IAM, SA

data "aws_route53_zone" "ep_zone" {
  count = var.use_existing_route53_zone ? 1 : 0

  name         = var.diom_domain
  private_zone = false
}

resource "aws_route53_zone" "ep_zone" {
  count = var.use_existing_route53_zone ? 0 : 1

  name = var.diom_domain
}
