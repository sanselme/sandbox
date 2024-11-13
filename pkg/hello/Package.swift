// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "hello",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "hellod", targets: ["server"]),
    .executable(name: "hellocli", targets: ["client"]),
  ],
  dependencies: [
    .package(path: "../api"),
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
  ],
  targets: [
    .executableTarget(
      name: "server",
      dependencies: [
        .product(name: "v1", package: "api"),
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
    .executableTarget(
      name: "client",
      dependencies: [
        .product(name: "v1", package: "api"),
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
  ]
)
