# configures ACM and externalDNS IAM, SA

resource "aws_route53_zone" "ep_zone" {
  name = var.diom_domain
}
