resource "kubernetes_namespace_v1" "ext_dns_ns" {
  metadata {
    name = local.ns_external_dns
  }
}

module "eks_sa_role_ext_dns" {
  source  = "terraform-aws-modules/iam/aws//modules/iam-role-for-service-accounts"
  version = "~> 6.4"

  name            = local.sa_ext_dns_name
  use_name_prefix = true

  attach_external_dns_policy    = true
  external_dns_hosted_zone_arns = var.dns_zone_arns

  oidc_providers = {
    ex = {
      provider_arn               = "${var.k8s_oidc_provider_arn}"
      namespace_service_accounts = ["${kubernetes_namespace_v1.ext_dns_ns.metadata[0].name}:${local.sa_ext_dns_name}"]
    }
  }
}

resource "kubernetes_service_account_v1" "sa_external_dns" {
  metadata {
    name      = local.sa_ext_dns_name
    namespace = kubernetes_namespace_v1.ext_dns_ns.metadata[0].name

    annotations = {
      "eks.amazonaws.com/role-arn" = module.eks_sa_role_ext_dns.arn
    }
  }
}

# install externalDNS for ingress mapping
# doc: https://github.com/kubernetes-sigs/external-dns
#

resource "helm_release" "external_dns" {
  name = local.sa_ext_dns_name

  repository = "https://kubernetes-sigs.github.io/external-dns/"
  chart      = "external-dns"
  version    = "1.20.0"

  namespace = kubernetes_namespace_v1.ext_dns_ns.metadata[0].name

  values = [
    yamlencode({
      domainFilters = [
        var.k8s_cluster_name
      ]

      serviceAccount = {
        create = false,
        name   = kubernetes_service_account_v1.sa_external_dns.metadata[0].name
      }

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
