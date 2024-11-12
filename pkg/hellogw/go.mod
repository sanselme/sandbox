module hellogw

go 1.23.2

require (
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.23.0
	google.golang.org/grpc v1.68.0
)

require (
	golang.org/x/net v0.29.0 // indirect
	golang.org/x/sys v0.25.0 // indirect
	golang.org/x/text v0.19.0 // indirect
	google.golang.org/genproto/googleapis/api v0.0.0-20241021214115-324edc3d5d38 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20241021214115-324edc3d5d38 // indirect
	google.golang.org/protobuf v1.35.1 // indirect
)

replace github.com/sanselme/sandbox/api v0.1.0 => ../api

replace google.golang.org/genproto v0.0.0-20220822174746-9e6da59bd2fc => google.golang.org/genproto v0.0.0-20241021214115-324edc3d5d38
