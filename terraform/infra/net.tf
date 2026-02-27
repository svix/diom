module "net" {
  source = "./modules/networking"
  providers = {
    aws    = aws,
    local  = local,
    tls    = tls,
    random = random,
    # datadog = datadog
  }

  env         = var.env
  aws_region  = data.aws_region.current.region
  name_prefix = local.name_prefix

  # dns
  use_existing_route53_zone = var.use_existing_route53_zone
  coyote_domain             = var.zone_name
  api_domain_prefix         = var.api_domain_prefix
  frontend_domain_prefix    = var.frontend_domain_prefix
  alb_log_expiration_days   = var.alb_log_expiration_days

  # network
  vpc_cidr = var.vpc_cidr

  tags = local.tags
}
