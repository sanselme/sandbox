module hellogw

go 1.24.0

require (
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.24.0
	github.com/spf13/cobra v1.8.1
	google.golang.org/grpc v1.79.3
)

require (
	github.com/inconshreveable/mousetrap v1.1.0 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
	golang.org/x/net v0.48.0 // indirect
	golang.org/x/sys v0.39.0 // indirect
	golang.org/x/text v0.32.0 // indirect
	google.golang.org/genproto/googleapis/api v0.0.0-20251202230838-ff82c1b0f217 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20251202230838-ff82c1b0f217 // indirect
	google.golang.org/protobuf v1.36.10 // indirect
)

replace github.com/sanselme/workspace/api v0.1.0 => ../../api

replace github.com/sanselme/workspace/openapi v0.1.0 => ../../docs/openapi
