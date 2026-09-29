resource "kubernetes_namespace_v1" "alb_ctr_ns" {
  metadata {
    name = local.ns_alb_ctr
  }
}

module "eks_sa_role_lb_ctrl" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts"
  version = "~> 6.4"

  name            = local.sa_alb_ctr_name
  use_name_prefix = true

  attach_load_balancer_controller_policy = true

  oidc_providers = {
    main = {
      provider_arn               = var.k8s_oidc_provider_arn
      namespace_service_accounts = ["${kubernetes_namespace_v1.alb_ctr_ns.metadata[0].name}:${local.sa_alb_ctr_name}"]
    }
  }
}

resource "kubernetes_service_account_v1" "sa_aws_lb_ctrl" {
  metadata {
    name      = local.sa_alb_ctr_name
    namespace = kubernetes_namespace_v1.alb_ctr_ns.metadata[0].name
    annotations = {
      "eks.amazonaws.com/role-arn" = "${module.eks_sa_role_lb_ctrl.arn}"
    }
  }
}

# install aws load balancer controller / fargate configuration
# doc: https://docs.aws.amazon.com/eks/latest/userguide/aws-load-balancer-controller.html
#

resource "helm_release" "aws_load_balancer_controller" {

  name = local.sa_alb_ctr_name

  repository = "https://aws.github.io/eks-charts"
  chart      = "aws-load-balancer-controller"
  version    = "3.1.0"

  namespace = kubernetes_namespace_v1.alb_ctr_ns.metadata[0].name

  values = [
    yamlencode({
      clusterName = var.k8s_cluster_name

      serviceAccount = {
        create = false,
        name   = kubernetes_service_account_v1.sa_aws_lb_ctrl.metadata[0].name
      }

      region = var.aws_region,
      vpcId  = var.vpc_id,

      tolerations = [
        {
          key      = "CriticalAddonsOnly"
          operator = "Exists"
          effect   = "NoSchedule"
        }
      ]
    })
  ]
}
