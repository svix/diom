resource "kubernetes_storage_class_v1" "ebs_gp3" {
  metadata {
    name = "${local.name_prefix}-gp3"

    annotations = {
      "storageclass.kubernetes.io/is-default-class" = "true"
    }
  }

  storage_provisioner = "ebs.csi.aws.com"

  parameters = {
    type      = "gp3"
    encrypted = "true"
  }

  volume_binding_mode    = "WaitForFirstConsumer"
  allow_volume_expansion = true
}
