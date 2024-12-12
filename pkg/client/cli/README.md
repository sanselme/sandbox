# Hello CLI

## Run

```bash
cd pkg/client/cli
swift run hellocli --help
```

## Build

```bash
swift build -c release --static-swift-stdlib --package-path pkg/client/cli
pkg/client/cli/.build/release/hellocli --help
```
