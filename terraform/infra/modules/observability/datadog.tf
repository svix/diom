
locals {
  datadog_enabled = var.datadog_api_key != "" && var.datadog_app_key != ""
}

resource "kubernetes_namespace_v1" "datadog" {
  count = local.datadog_enabled ? 1 : 0

  metadata {
    name = "datadog"
  }
}

resource "helm_release" "datadog-operator" {
  count = local.datadog_enabled ? 1 : 0

  namespace  = "datadog"
  name       = "datadog"
  chart      = "datadog-operator"
  repository = "https://helm.datadoghq.com"

  create_namespace = false
}

resource "kubernetes_secret_v1" "datadog-secret" {
  count = local.datadog_enabled ? 1 : 0

  metadata {
    namespace = kubernetes_namespace_v1.datadog[0].metadata[0].name
    name      = "datadog-secret"
  }

  data = {
    "api-key" = var.datadog_api_key
    "app-key" = var.datadog_app_key
  }
}

resource "kubectl_manifest" "datadog" {
  count = local.datadog_enabled ? 1 : 0

  depends_on = [
    kubernetes_secret_v1.datadog-secret,
    helm_release.datadog-operator
  ]

  yaml_body = yamlencode({
    apiVersion = "datadoghq.com/v2alpha1"
    kind       = "DatadogAgent"
    metadata = {
      name      = "${var.name_prefix}-dd-agent"
      namespace = kubernetes_namespace_v1.datadog[0].metadata[0].name
    }
    spec = {
      global = {
        clusterName = "${var.name_prefix}-dd"
        site        = "datadoghq.com"
        tags = [
          "env:${var.env}"
        ]
        registry = "public.ecr.aws/datadog"
        credentials = {
          apiSecret = {
            secretName = kubernetes_secret_v1.datadog-secret[0].metadata[0].name
            keyName    = "api-key"
          }
          appSecret = {
            secretName = kubernetes_secret_v1.datadog-secret[0].metadata[0].name
            keyName    = "app-key"
          }
        }
      },
      features = {
        apm                  = { enabled = false }
        usm                  = { enabled = false }
        kubeStateMetricsCore = { enabled = true }
        prometheusScrape     = { enabled = false }
      }
      override = {
        nodeAgent = {
          tolerations = [
            {
              key      = "dedicated",
              operator = "Exists",
              effect   = "NoSchedule"
            }
          ]
        }
      }
    }
  })
}
