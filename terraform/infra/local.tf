################################################################################
# Data
################################################################################

data "aws_region" "current" {}

data "aws_caller_identity" "current" {}

data "aws_elb_service_account" "main" {}


locals {

  name_prefix = "${var.env}-coyote"

  api_endpoint            = "${var.api_domain_prefix}.${var.zone_name}"
  static_endpoint         = "${var.frontend_domain_prefix}.${var.zone_name}"
  frontend_endpoint       = "frontend.${var.zone_name}"
  app_portal_service_name = "svix-coyote"

  tags = {
    Env     = var.env,
    Source  = "terraform",
    Project = "coyote"
  }
}
