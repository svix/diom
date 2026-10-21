
resource "kubernetes_namespace_v1" "this" {
  count = local.is_prod ? 1 : 0
  metadata {
    name = var.app_namespace
  }
}

resource "helm_release" "this" {
  depends_on = [kubernetes_namespace_v1.this]

  name             = "${var.name_prefix}-diom"
  chart            = "${path.module}/helm/chart/operator/"
  namespace        = local.is_prod ? kubernetes_namespace_v1.this[0].metadata[0].name : "diom-db"
  create_namespace = false

  values = [
    jsonencode({
      replicaCount = 1
      logLevel     = "debug"
    })
  ]
}
