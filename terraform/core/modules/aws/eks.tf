################################################################################
# EKS
################################################################################

resource "aws_kms_key" "eks" {
  description             = "EKS Secret Encryption Key"
  deletion_window_in_days = 7
  enable_key_rotation     = true
}

# public access is needed
#tfsec:ignore:aws-eks-no-public-cluster-access
#tfsec:ignore:aws-eks-no-public-cluster-access-to-cidr
module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 21.15"

  name                    = local.name
  kubernetes_version      = var.k8s_version
  endpoint_private_access = true
  endpoint_public_access  = true

  addons = {
    coredns = {
      configuration_values = jsonencode({
        computeType = "Fargate"
      })
    }
    kube-proxy = {}
    vpc-cni    = {}
  }

  encryption_config = {
    provider_key_arn = aws_kms_key.eks.arn
    resources        = ["secrets"]
  }

  vpc_id                   = module.vpc.vpc_id
  subnet_ids               = module.vpc.private_subnets
  control_plane_subnet_ids = module.vpc.intra_subnets

  create_security_group                  = false
  create_node_security_group             = false
  enabled_log_types                      = ["audit", "api", "authenticator", "controllerManager", "scheduler"]
  cloudwatch_log_group_retention_in_days = 14

  eks_managed_node_groups = {
    "system-node-group-0" = {
      instance_types                 = var.system_instance_types,
      min_size                       = var.system_min_node_count,
      max_size                       = var.system_max_node_count,
      desired_size                   = var.system_desired_node_count,
      ami_type                       = var.system_ami_type,
      enable_efa_only                = false, # this is enabled by default and might not allow common instance types?
      use_latest_ami_release_version = true,
      subnet_ids                     = aws_subnet.eks_system[*].id
      taints = {
        critical_addons = {
          key    = "CriticalAddonsOnly",
          value  = "true",
          effect = "NO_SCHEDULE"
        }
      }
    },
    "svix-node-group-0" = {
      instance_types                 = var.app_instance_types,
      min_size                       = var.app_min_node_count,
      max_size                       = var.app_max_node_count,
      desired_size                   = var.app_desired_node_count,
      ami_type                       = var.app_ami_type,
      enable_efa_only                = false, # this is enabled by default and might not allow common instance types?
      use_latest_ami_release_version = true,
      subnet_ids                     = module.vpc.private_subnets
    }
  }

  # auth
  authentication_mode                      = "API_AND_CONFIG_MAP"
  enable_cluster_creator_admin_permissions = true

  access_entries = merge(
    local.admin_users,
    local.admin_roles,
  )
}

#tfsec:ignore:aws-s3-enable-versioning
#tfsec:ignore:aws-s3-enable-bucket-logging
resource "aws_s3_bucket" "alb_log_bucket" {
  bucket = "${var.name_prefix}-alb"

  force_destroy = false

  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_s3_bucket_public_access_block" "alb_log_bucket" {
  bucket = aws_s3_bucket.alb_log_bucket.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

#tfsec:ignore:aws-s3-encryption-customer-key
resource "aws_s3_bucket_server_side_encryption_configuration" "alb_log_bucket" {
  bucket = aws_s3_bucket.alb_log_bucket.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}


resource "aws_s3_bucket_lifecycle_configuration" "alb_log_bucket_config" {
  bucket = aws_s3_bucket.alb_log_bucket.id

  rule {
    id = "log_expiry"

    expiration {
      days = var.alb_log_expiration_days
    }

    status = "Enabled"
  }
}

resource "aws_s3_bucket_policy" "alb_bucket_policy" {
  bucket = aws_s3_bucket.alb_log_bucket.id
  policy = data.aws_iam_policy_document.alb_iam_policy.json
}

data "aws_iam_policy_document" "alb_iam_policy" {
  statement {
    principals {
      type        = "AWS"
      identifiers = [var.elb_service_account_arn]
    }

    actions = [
      "s3:PutObject",
    ]

    resources = [
      aws_s3_bucket.alb_log_bucket.arn,
      "${aws_s3_bucket.alb_log_bucket.arn}/api/AWSLogs/${var.account_id}/*",
      "${aws_s3_bucket.alb_log_bucket.arn}/app/AWSLogs/${var.account_id}/*",
      "${aws_s3_bucket.alb_log_bucket.arn}/frontend/AWSLogs/${var.account_id}/*",
    ]
  }
}
