# hacking

## cluster

```bash
kind create cluster --config config/kind.yaml
kustomize build deployment/crd | kubectl apply -f -

docker container run --rm \
  --network kind \
  -v /var/run/docker.sock:/var/run/docker.sock \
  sanselme/cloud-provider-kind

# docker container run --rm \
#   --name kind-proxy \
#   --network kind \
#   -p 1080:1080 \
#   serjs/go-socks5-proxy@sha256:aad36c623f16850d7cea0171d1aa79d706129191db9e270b6dfd7db6b552c734
# export ALL_PROXY=socks5://localhost:1080

# crds
kustomize build "https://github.com/kubernetes-sigs/gateway-api/config/crd/experimental?ref=v1.2.0" | kubectl apply -f -
kustomize build "https://github.com/kubernetes-csi/external-snapshotter/client/config/crd?ref=v8.1.0" | kubectl apply -f -
```

## cni

```bash
# cilium
cat <<eof | helm upgrade cilium \
  --atomic \
  --install cilium \
  --repo https://helm.cilium.io/ \
  --version 1.16.2 \
  -n kube-system \
  -f -
---
kubeProxyReplacement: true
l2announcements:
  enabled: true
gatewayAPI:
  enabled: true
operator:
  replicas: 1
eof

# cni resource
cat <<eof | helm upgrade cni-resource \
  --atomic \
  --install oci://registry-1.docker.io/sanselmechart/cni-resource \
  --version 0.1.0 \
  -n kube-system \
  -f -
---
cilium:
  pool:
    allowFirstLastIPs: "'No'"
  advert:
    l2:
      enabled: true
    bgp:
      enabled: false
  config:
    cluster:
      enabled: false
    peer:
      enabled: false
eof
```

## cert-manager

```bash
# cert-manager
cat <<eof | helm upgrade cert-manager \
  --atomic \
  --create-namespace \
  --install cert-manager \
  --repo https://charts.jetstack.io/ \
  --version v1.16.0 \
  -n cert-manager \
  -f -
---
crds:
  enabled: true
config:
  apiVersion: controller.config.cert-manager.io/v1alpha1
  kind: ControllerConfiguration
  enableGatewayAPI: true
eof

# cluster-issuer
cat <<eof | helm upgrade ca-clusterissuer \
  --atomic \
  --install oci://registry-1.docker.io/sanselmechart/ca-clusterissuer \
  --version 0.2.0 \
  -n cert-manager \
  -f -
---
manifests:
  clusterissuer: false
  selfsigned: true
eof
```

## gateway

```bash
kustomize build hack/gateway | kubectl apply -f -
```

## knative

```bash
# operator
helm upgrade knative-operator \
  --atomic \
  --create-namespace \
  --install oci://registry-1.docker.io/sanselmechart/knative-operator \
  --version 0.3.0 \
  -n operators

# serving
cat <<eof | helm upgrade knative-serving \
  --atomic \
  --install oci://registry-1.docker.io/sanselmechart/knative-serving \
  --version 0.3.0 \
  -n knative \
  -f -
---
ca-issuer:
  enabled: true
knative:
  ingress:
    type: gateway-api
eof

# net-gateway
kustomize build hack/knative/net-gateway | kubectl apply -f -

# eventing
cat <<eof | helm upgrade knative-eventing \
  --atomic \
  --install oci://registry-1.docker.io/sanselmechart/knative-eventing \
  --version 0.3.0 \
  -n knative \
  -f -
---
sources:
  # ceph:
  #   enabled: true
  github:
    enabled: true
  # fixme: failing due to rbac
  # rabbitmq:
  #   enabled: true
  redis:
    enabled: true
eof

# backstage
kustomize build hack/knative/backstage | kubectl apply -f -

# resource
```

## rabbitmq

```bash
# operator
cat <<eof | helm upgrade \
  --atomic \
  --install oci://registry-1.docker.io/bitnamicharts/rabbitmq-cluster-operator \
  --version 4.3.27 \
  -n operators \
  -f -
---
clusterOperator:
  metrics:
    service:
      enabled: true
    # serviceMonitor:
    #   enabled: true
    # podMonitor:
    #   enabled: true
msgTopologyOperator:
  metrics:
    service:
      enabled: true
    # serviceMonitor:
    #   enabled: true
    # podMonitor:
    #   enabled: true
useCertManager: true
eof

# cluster
kustomize build hack/knative/rabbitmq/cluster | kubectl apply -f -

# broker
kustomize build hack/knative/rabbitmq/broker | kubectl apply -f -

# source
kustomize build hack/knative/rabbitmq/source | kubectl apply -f -

# service
kustomize build hack/knative/rabbitmq | kubectl apply -f -
```
