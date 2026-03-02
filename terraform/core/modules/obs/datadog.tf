locals {
  datadogEnabled = var.datadog_api_key != "" && var.datadog_app_key != ""
}

resource "kubernetes_namespace" "datadog" {
  count = local.datadogEnabled ? 1 : 0

  metadata {
    name = "datadog"
  }
}

resource "helm_release" "datadog-operator" {
  count = local.datadogEnabled ? 1 : 0

  namespace  = "datadog"
  name       = "datadog"
  chart      = "datadog-operator"
  repository = "https://helm.datadoghq.com"

  create_namespace = false
}

resource "kubernetes_secret" "datadog-secret" {
  count = local.datadogEnabled ? 1 : 0

  metadata {
    namespace = kubernetes_namespace.datadog[0].metadata[0].name
    name      = "datadog-secret"
  }

  data = {
    "api-key" = var.datadog_api_key
    "app-key" = var.datadog_app_key
  }
}

resource "kubectl_manifest" "datadog" {
  count = local.datadogEnabled ? 1 : 0

  depends_on = [
    kubernetes_secret.datadog-secret,
    helm_release.datadog-operator
  ]

  yaml_body = yamlencode({
    apiVersion = "datadoghq.com/v2alpha1"
    kind       = "DatadogAgent"
    metadata = {
      name      = "datadog"
      namespace = kubernetes_namespace.datadog[0].metadata[0].name
    }
    spec = {
      global = {
        clusterName = var.cluster_name
        site        = "datadoghq.com"
        tags = [
          "env:${var.env}"
        ]
        registry = "public.ecr.aws/datadog"
        credentials = {
          apiSecret = {
            secretName = kubernetes_secret.datadog-secret[0].metadata[0].name
            keyName    = "api-key"
          }
          appSecret = {
            secretName = kubernetes_secret.datadog-secret[0].metadata[0].name
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
