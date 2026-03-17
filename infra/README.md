## Current K8s Deployment Process

```bash
# Get the current CRD
cargo run -p coyote-operator -- --print-crd > ./infra/crd.yaml

# Install the CRD
kubectl apply -f crd.yaml

# Build the operator Docker image
... This is all manual and hacked together at this point via ECR

# Install the operator
helm install coyote-operator ./helm-operator --set-string image.tag={tag of operator Docker image}

# Install the Cluster CR
kubectl apply -f cluster.yaml

# Look at your big, beautiful Coyote cluster running in a k8s cluster
```
