// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "hellosock",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "hellosock", targets: ["websocket"]),
  ],
  dependencies: [
    .package(path: "../../../api"),
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
  ],
  targets: [
    .executableTarget(
      name: "websocket",
      dependencies: [
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
        .product(name: "v1", package: "api"),
      ]
    ),
  ]
)
