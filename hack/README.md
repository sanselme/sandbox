# hacking

```bash
# kind cluster
./tools/kind

# minikube cluster
./tools/minikube

# crds
kubectl apply -k build deployment/crd
```

## cni

```bash
# cilium
cat <<eof | helm upgrade cilium \
  --atomic \
  --install \
  --version 1.16.4 \
  --wait \
  -n kube-system \
  oci://registry-1.docker.io/sanselmechart/cilium \
  -f -
---
k8sServiceHost: localhost
k8sServicePort: 6443
kubeProxyReplacement: true
ipam:
  mode: kubernetes
gatewayAPI:
  enabled: true
  gatewayClass:
    create: "true"
  hostNetwork:
    enabled: true
operator:
  replicas: 1
eof

# cni resource
cat <<eof | helm upgrade cni-resource \
  --atomic \
  --install \
  --version 0.1.0 \
  --wait \
  -n kube-system \
  oci://registry-1.docker.io/sanselmechart/cni-resource \
  -f -
---
cilium:
  pool:
    allowFirstLastIPs: "'No'"
  advert:
    l2:
      enabled: false
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
  --install \
  --version v1.16.2 \
  --wait \
  -n cert-manager \
  oci://registry-1.docker.io/sanselmechart/cert-manager \
  -f -
---
crds:
  enabled: true
config:
  apiVersion: controller.config.cert-manager.io/v1alpha1
  kind: ControllerConfiguration
  featureGates:
    ExperimentalGatewayAPISupport: true
eof

# cluster-issuer
cat <<eof | helm upgrade ca-clusterissuer \
  --atomic \
  --install oci://registry-1.docker.io/sanselmechart/ca-clusterissuer \
  --version 0.2.0 \
  --wait \
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
# envoy gateway (if not using cilium, eg. minikube)
helm upgrade envoy-gateway \
  --atomic \
  --install oci://docker.io/envoyproxy/gateway-helm \
  --version v1.2.3 \
  -n kube-system
cat <<eof | kubectl apply -f -
---
apiVersion: gateway.networking.k8s.io/v1
kind: GatewayClass
metadata:
  name: envoy
spec:
  controllerName: gateway.envoyproxy.io/gatewayclass-controller
eof
sed -e 's/cilium/envoy/g' build hack/gateway | kubectl apply -k -

# cilium gateways
kubectl apply -k build hack/gateway
```

## knative

```bash
# operator
helm upgrade knative-operator \
  --atomic \
  --create-namespace \
  --install \
  --version 0.3.0 \
  --wait \
  -n operators \
  oci://registry-1.docker.io/sanselmechart/knative-operator

# serving
cat <<eof | helm upgrade knative-serving \
  --atomic \
  --install \
  --version 0.3.0 \
  --wait \
  -n knative \
  oci://registry-1.docker.io/sanselmechart/knative-serving \
  -f -
---
ca-issuer:
  enabled: true
knative:
  domain: svc.kube.local
  ingress:
    type: gateway-api
eof

# net-gateway
kubectl apply -k build hack/knative/net-gateway
# todo: update config-gateway
# external-gateways defines the Gateway to be used for external traffic
# external-gateways: |
#   - class: envoy
#     gateway: kube-system/knative
#     service: kube-system/envoy-kube-system-knative-d5294fcd
#     supported-features:
#     - HTTPRouteRequestTimeout
# local-gateways defines the Gateway to be used for cluster local traffic
# local-gateways: |
#   - class: envoy
#     gateway: kube-system/knative
#     service: kube-system/envoy-kube-system-knative-d5294fcd
#     supported-features:
#     - HTTPRouteRequestTimeout

# eventing
cat <<eof | kubectl apply -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: rabbitmq-clusterrole
rules:
  - apiGroups: ["rabbitmq.com"]
    resources: ["bindings", "bindings/status", "exchanges", "exchanges/status", "queues", "queues/status", "rabbitmqclusters"]
    verbs: ["create", "delete", "get", "list", "patch", "update", "watch"]
eof
cat <<eof | kubectl apply -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: rabbitmq-clusterrolebinding
subjects:
  - kind: ServiceAccount
    name: knative-operator
    namespace: operators
roleRef:
  kind: ClusterRole
  name: rabbitmq-clusterrole
  apiGroup: rbac.authorization.k8s.io
eof
cat <<eof | helm upgrade knative-eventing \
  --atomic \
  --install \
  --version 0.3.0 \
  --wait \
  -n knative \
  oci://registry-1.docker.io/sanselmechart/knative-eventing \
  -f -
---
sources:
  # ceph:
  #   enabled: true
  github:
    enabled: true
  rabbitmq:
    enabled: true
  redis:
    enabled: true
eof

# backstage
kubectl apply -k build hack/knative/backstage
```

## rabbitmq

```bash
# operator
cat <<eof | helm upgrade rabbitmq-operator \
  --atomic \
  --install \
  --version 4.3.27 \
  --wait \
  -n operators \
  oci://registry-1.docker.io/bitnamicharts/rabbitmq-cluster-operator \
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
kubectl apply -k hack/knative/rabbitmq/cluster

# broker
kubectl apply -k hack/knative/rabbitmq/broker

# source
kubectl apply -k hack/knative/rabbitmq/source

# service
kubectl apply -k hack/knative/rabbitmq
```

## libvirt

```bash
source scripts/environment.sh

# create cloudinit iso
sudo -E createiso /var/lib/libvirt/boot/cloudinit.iso hack/meta-data hack/user-data hack/network-config

# create volume
sudo -E createvol /var/lib/libvirt/images/sandbox.qcow2 noble-server-cloudimg-arm64.img

# create vm
sudo -E virsh define hack/vm.xml
sudo -E virsh start sandbox
```
