# Hello Event

## Run

```bash
cd pkg/service/helloce
go run . --help
```

## Build

```bash
go build -o helloce pkg/service/helloce
helloce --help
```

## Docker

[Dockerfile](https://github.com/anselmes/images/blob/hello/build/img/hello/Dockerfile.cloudevent)

```bash
# build container image
git clone https://github.com/anselmes/images --single-branch --depth 1 --branch hello
docker buildx build --platform linux/amd64,linux/arm64,linux/riscv64 -f build/img/hello/Dockerfile.cloudevent -t helloce:0.1.0 .

# run container
docker container run --rm -it hellod:0.1.0 /helloce --help
```

## Kubernetes

```bash
# deploy on kubernetes
cat <<eof | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: hello-event
spec:
  containers:
    - name: hellod
      image: helloce:0.1.0
eof

# deploy using knative
cat <<eof | kubectl apply -f -
---
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: hello-event
  namespace: sandbox
spec:
  template:
    spec:
      containers:
        - image: helloce:0.1.0
          ports:
            - containerPort: 8080
eof
```

## Usage

```bash
curl http://hello-event:8080 \
  -H "ce-id: 1" \
  -H "ce-source: curl" \
  -H "ce-specversion: 1.0" \
  -H "ce-type: hello" \
  -H "content-type: application/json" \
  -X POST \
  -d '{"name":""}'
# {"message":"Hello, stranger"}
```
