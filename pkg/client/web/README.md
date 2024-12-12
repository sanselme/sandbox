# Hello Web

## Build and Run

```bash
swift build -c release --static-swift-stdlib --package-path pkg/client/helloweb
pkg/client/helloweb/.build/release/hellod
cd pkg/client/helloweb/Build
python3 -m http.server -d . -p 8080
```

## Docker

[Dockerfile](https://github.com/anselmes/images/blob/hello/build/img/hello/Dockerfile.webclient)

```bash
# build container image
git clone https://github.com/anselmes/images --single-branch --depth 1 --branch hello
docker buildx build --platform linux/amd64,linux/arm64,linux/riscv64 -f build/img/hello/Dockerfile.webclient -t helloweb:0.1.0 .

# run container
docker container run --rm -p 8080:80 -it helloweb:0.1.0
```

## Kubernetes

```bash
# deploy on kubernetes
cat <<eof | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: hello-web
spec:
  containers:
    - name: hellod
      image: helloweb:0.1.0
eof

# deploy using knative
cat <<eof | kubectl apply -f -
---
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: hello-web
  namespace: sandbox
spec:
  template:
    spec:
      containers:
        - image: helloweb:0.1.0
          ports:
            - containerPort: 8080
eof
```

## Usage

[Web Client](http://hello-web:8080)
