# Hello Gateway

## Run

```bash
cd pkg/gateway
go run . --help
```

## Build

```bash
go build -o hellogw pkg/gateway
hellogw --help
```

## Docker

[Dockerfile](https://github.com/anselmes/images/blob/hello/build/img/hello/Dockerfile.cloudevent)

```bash
# build container image
git clone https://github.com/anselmes/images --single-branch --depth 1 --branch hello
docker buildx build --platform linux/amd64,linux/arm64,linux/riscv64 -f build/img/hello/Dockerfile.openapi -t hellogw:0.1.0 .

# run container
docker container run --rm -it hellod:0.1.0 /hellogw --help
```

## Kubernetes

```bash
# deploy on kubernetes
cat <<eof | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: hello-gateway
spec:
  containers:
    - name: hellod
      image: hellogw:0.1.0
eof

# deploy using knative
cat <<eof | kubectl apply -f -
---
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: hello-gateway
  namespace: sandbox
spec:
  template:
    spec:
      containers:
        - image: hellogw:0.1.0
          ports:
            - containerPort: 8080
eof
```

## Usage

```bash
curl http://hello-gateway:8080 \
  -H "content-type: application/json" \
  -X POST \
  -d '{"name":""}'
# {"message":"Hello, stranger"}
```
