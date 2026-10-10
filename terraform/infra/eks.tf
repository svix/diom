module "eks" {

  source = "./modules/eks"
  providers = {
    aws    = aws,
    local  = local,
    tls    = tls,
    random = random,
    # datadog = datadog
  }

  env                     = var.env
  account_id              = data.aws_caller_identity.current.account_id
  elb_service_account_arn = data.aws_elb_service_account.main.arn
  name_prefix             = module.net.name_prefix

  aws_region               = module.net.aws_region
  vpc_id                   = module.net.vpc_id
  private_subnet_ids       = module.net.private_subnet_ids
  database_subnet_ids      = module.net.database_subnet_ids
  diom_namespace         = var.diom_namespace
  control_plane_subnet_ids = module.net.intra_subnet_ids
  eks_subnet_ids           = module.net.eks_subnet_ids
  dns_zone_arns            = module.net.dns_zone_arns
  dns_zone_name            = module.net.dns_zone_name

  # auth
  admin_users = var.admin_users
  admin_roles = var.admin_roles

  # eks managed node groups
  k8s_version               = var.k8s_version
  system_instance_types     = var.system_instance_types
  system_min_node_count     = var.system_min_node_count
  system_max_node_count     = var.system_max_node_count
  system_desired_node_count = var.system_desired_node_count
  system_ami_type           = var.system_ami_type

  app_instance_types     = var.app_instance_types
  app_min_node_count     = var.app_min_node_count
  app_max_node_count     = var.app_max_node_count
  app_desired_node_count = var.app_desired_node_count
  app_ami_type           = var.app_ami_type

  db_instance_types     = var.db_instance_types
  db_min_node_count     = var.db_min_node_count
  db_max_node_count     = var.db_max_node_count
  db_desired_node_count = var.db_desired_node_count
  db_ami_type           = var.db_ami_type

  alb_log_expiration_days = var.alb_log_expiration_days

  pagerduty_service = var.pagerduty_service

  tags = local.tags
}
