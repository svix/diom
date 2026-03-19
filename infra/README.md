## Current K8s Deployment Process

```sh
export DIOM_IMAGE_TAG=sha-f6e38276dc0f6f583aba7d916e57585915fe8b2a

# Generate the CRD, if needed:

# Install the Helm chart -- right now this chart is everything in one: CRD, operator, and cluster spec. Must currently be installed from a local path:
helm upgrade --install diom ./helm-diom --set operator.image.tag=$DIOM_IMAGE_TAG --set cluster.image.tag=${DIOM_IMAGE_TAG}

# Look at your big, beautiful Diom cluster running in a k8s cluster

# Note that upgrading the CRD complicates things. Look at the chart README for details there.
```
