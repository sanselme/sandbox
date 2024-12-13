// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "hellocli",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "hellocli", targets: ["client"])
  ],
  dependencies: [
    .package(path: "../../../api"),
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
    .package(url: "https://github.com/swift-server/async-http-client.git", from: "1.24.0"),
  ],
  targets: [
    .executableTarget(
      name: "client",
      dependencies: [
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
        .product(name: "AsyncHTTPClient", package: "async-http-client"),
        .product(name: "v1", package: "api"),
      ]
    )
  ]
)
