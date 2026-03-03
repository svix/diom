module "obs" {
  source = "./modules/observability"
  providers = {
    aws    = aws,
    local  = local,
    tls    = tls,
    random = random,
    # datadog = datadog
  }

  env         = var.env
  aws_region  = data.aws_region.current.region
  account_id  = data.aws_caller_identity.current.account_id
  name_prefix = random_pet.client_prefix.id

  datadog_api_key = var.datadog_api_key
  datadog_app_key = var.datadog_app_key

  pagerduty_service = var.pagerduty_service
}
