// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "hellod",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "hellod", targets: ["daemon"])
  ],
  dependencies: [
    .package(path: "../../../api"),
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
  ],
  targets: [
    .executableTarget(
      name: "daemon",
      dependencies: [
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
        .product(name: "v1", package: "api"),
      ]
    )
  ]
)
