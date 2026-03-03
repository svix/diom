module "obs" {
  source = "./modules/observability"
  providers = {
    aws    = aws,
    local  = local,
    tls    = tls,
    random = random,
    # datadog = datadog
  }

  env                     = var.env
  aws_region              = data.aws_region.current.region
  account_id              = data.aws_caller_identity.current.account_id

  pagerduty_service = var.pagerduty_service

  tags = local.tags
}
