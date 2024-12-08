// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "api",
  products: [
    .library(name: "v1", targets: ["v1"]),
  ],
  dependencies: [
    .package(url: "https://github.com/grpc/grpc-swift-nio-transport.git", from: "1.0.0-alpha.1"),
    .package(url: "https://github.com/grpc/grpc-swift-protobuf.git", from: "1.0.0-alpha.1"),
    .package(url: "https://github.com/grpc/grpc-swift.git", from: "2.0.0-alpha.1"),
  ],
  targets: [
    .target(
      name: "v1",
      dependencies: [
        .product(name: "GRPCCore", package: "grpc-swift"),
        .product(name: "GRPCNIOTransportHTTP2", package: "grpc-swift-nio-transport"),
        .product(name: "GRPCProtobuf", package: "grpc-swift-protobuf"),
      ],
      path: "./v1"
    ),
  ]
)
