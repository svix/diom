
resource "random_password" "grafana_admin" {
  length           = 24
  special          = true
  override_special = "!@#%^*-_=+"
}

resource "kubernetes_namespace_v1" "monitoring" {
  metadata {
    name = "monitoring"
  }
}

resource "kubernetes_secret_v1" "grafana_admin" {
  metadata {
    name      = "${var.name_prefix}-grafana"
    namespace = kubernetes_namespace_v1.monitoring.metadata[0].name
  }

  type = "Opaque"

  data = {
    username = "admin"
    password = random_password.grafana_admin.result
  }
}

#ToDo: Dedicated AZ for monitoring/node affinity based on availability zone
#    : for ebs vicinity
resource "helm_release" "kube_prometheus_stack" {
  name             = "${var.name_prefix}-kube-prometheus-stack"
  namespace        = kubernetes_namespace_v1.monitoring.metadata[0].name
  create_namespace = false

  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "kube-prometheus-stack"
  version    = "82.6.0"

  values = [
    yamlencode({

      grafana = {
        admin = {
          existingSecret = kubernetes_secret_v1.grafana_admin.metadata[0].name
          userKey        = "username"
          passwordKey    = "password"
        }

        persistence = {
          enabled      = true
          storageClass = "gp3"
          size         = "10Gi"
        }
      }

      prometheus = {
        prometheusSpec = {
          retention = "15d"

          storageSpec = {
            volumeClaimTemplate = {
              spec = {
                storageClassName = "gp3"
                accessModes      = ["ReadWriteOnce"]
                resources = {
                  requests = {
                    storage = "50Gi"
                  }
                }
              }
            }
          }
        }
      }

      alertmanager = {
        alertmanagerSpec = {
          storage = {
            volumeClaimTemplate = {
              spec = {
                storageClassName = "gp3"
                accessModes      = ["ReadWriteOnce"]
                resources = {
                  requests = {
                    storage = "10Gi"
                  }
                }
              }
            }
          }
        }
      }

    })
  ]
}
