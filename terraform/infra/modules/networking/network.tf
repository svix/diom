
################################################################################
# VPC
################################################################################

# tfsec needs fix. flow-log created through module
#tfsec:ignore:aws-ec2-require-vpc-flow-logs-for-all-vpcs
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 6.6"

  name = local.name
  cidr = var.vpc_cidr

  azs                          = local.azs
  private_subnets              = [local.subnet_cidrs[0], local.subnet_cidrs[1]]
  public_subnets               = [local.subnet_cidrs[2], local.subnet_cidrs[3]]
  database_subnets             = [local.subnet_cidrs[4], local.subnet_cidrs[5]]
  create_database_subnet_group = true

  enable_nat_gateway   = true
  single_nat_gateway   = true
  enable_dns_hostnames = true
  enable_dns_support   = true

  enable_flow_log                                 = true
  create_flow_log_cloudwatch_iam_role             = true
  create_flow_log_cloudwatch_log_group            = true
  flow_log_cloudwatch_log_group_retention_in_days = 14
  flow_log_cloudwatch_log_group_name_prefix       = "${var.name_prefix}-eks-"

  public_subnet_tags = {
    "kubernetes.io/cluster/${local.name}" = "shared"
    "kubernetes.io/role/elb"              = 1
  }

  private_subnet_tags = {
    "kubernetes.io/cluster/${local.name}" = "shared"
    "kubernetes.io/role/internal-elb"     = 1
  }
}
