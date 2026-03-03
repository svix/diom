################################################################################
# EKS
################################################################################

data "aws_subnet" "diom_subnet" {
  id = var.database_subnet_ids[0]
}

module "ebs_csi_irsa" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts"
  version = "~> 6.4"
  name    = "${local.eks_cluster_name}-ebs-csi-role"

  attach_ebs_csi_policy = true

  oidc_providers = {
    main = {
      provider_arn               = module.eks.oidc_provider_arn
      namespace_service_accounts = ["kube-system:ebs-csi-controller-sa"]
    }
  }
}

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

  name                    = local.eks_cluster_name
  kubernetes_version      = var.k8s_version
  endpoint_private_access = true
  endpoint_public_access  = true

  addons = {
    coredns = {
      most_recent = true
      configuration_values = jsonencode({
        computeType = "Fargate"
      })
    }

    kube-proxy = {
      most_recent = true
    }

    vpc-cni = {
      most_recent = true
    }

    aws-ebs-csi-driver = {
      most_recent              = true
      service_account_role_arn = module.ebs_csi_irsa.arn
    }
  }

  encryption_config = {
    provider_key_arn = aws_kms_key.eks.arn
    resources        = ["secrets"]
  }

  vpc_id                   = var.vpc_id
  subnet_ids               = var.private_subnet_ids
  control_plane_subnet_ids = var.control_plane_subnet_ids

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
      kubernetes_version             = var.k8s_version
      ami_type                       = var.system_ami_type,
      enable_efa_only                = false, # this is enabled by default and might not allow common instance types?
      use_latest_ami_release_version = true,
      subnet_ids                     = var.eks_subnet_ids
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
      kubernetes_version             = var.k8s_version
      ami_type                       = var.app_ami_type,
      enable_efa_only                = false, # this is enabled by default and might not allow common instance types?
      use_latest_ami_release_version = true,
      subnet_ids                     = var.private_subnet_ids
    }
    "diom-node-group-az0" = {
      instance_types                 = var.app_instance_types,
      min_size                       = var.app_min_node_count,
      max_size                       = var.app_max_node_count,
      desired_size                   = var.app_desired_node_count,
      kubernetes_version             = var.k8s_version
      ami_type                       = var.app_ami_type,
      enable_efa_only                = false, # this is enabled by default and might not allow common instance types?
      use_latest_ami_release_version = true,
      subnet_ids                     = [data.aws_subnet.diom_subnet.id]

      #ToDo: Do we want the nodes to be created close to each other in an availability zone?
      # create_placement_group         = true
      # placement = {
      #   availability_zone = data.aws_subnet.cayote_subnet.availability_zone
      #   group_name        = "${local.eks_cluster_name}-diom-pg-${data.aws_subnet.cayote_subnet.availability_zone}"
      # }
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
