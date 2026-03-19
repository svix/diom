## Current K8s Deployment Process

```bash
export DIOM_IMAGE_TAG=sha-f6e38276dc0f6f583aba7d916e57585915fe8b2a

# Get the current CRD
cargo run -p diom-operator -- --print-crd > ./crd.yaml

# Install the CRD
kubectl apply -f crd.yaml

# Install the operator -- right now the Helm chart
# must be installed from a local path:
helm upgrade --install diom-operator ./helm-diom-operator --set-string image.tag=$DIOM_IMAGE_TAG

# Install the Cluster CR (substitute the desired image tag)
envsubst < cluster.yaml | kubectl apply -f -

# Look at your big, beautiful Diom cluster running in a k8s cluster
```
