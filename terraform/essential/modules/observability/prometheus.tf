#tfsec:ignore:aws-sns-topic-encryption-use-cmk
resource "aws_sns_topic" "user_updates" {
  name              = "${var.name_prefix}-user-updates"
  kms_master_key_id = "alias/aws/sns"
}

module "prometheus" {
  source = "git::https://github.com/svix/monorepo-private.git//terraform-eks/modules/prometheus?ref=ashay/eks-prometheus-newer-provider"

  env                   = var.env
  region                = var.aws_region
  pagerduty_routing_key = var.pagerduty_routing_key
  eks_cluster_name      = var.k8s_cluster_name
  tsnet                 = var.tailscale_tsnet
  storage_class_name    = var.storage_class_name
  sns_alarm_topic_arn   = aws_sns_topic.user_updates.arn
}
