# Hello Web

## Build and Run

```bash
swift run --package-path pkg/client/helloweb
python3 -m http.server -d pkg/client/helloweb/Build -p http/2.0 8080
```

## Docker

[Dockerfile](https://github.com/anselmes/images/blob/hello/build/img/hello/Dockerfile.webclient)

```bash
# run container
docker container run --rm -p 8080:80 -v ./pkg/client/helloweb/Build:/usr/share/nginx/html -it nginx:1.27.3
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
