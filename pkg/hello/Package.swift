// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "hello",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "hello", targets: ["web"]),
    .executable(name: "hellocli", targets: ["client"]),
    .executable(name: "hellod", targets: ["server"]),
    .executable(name: "helloctl", targets: ["controller"]),
    .library(name: "hellolib", targets: ["lib"]),
  ],
  dependencies: [
    .package(path: "../api"),
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
  ],
  targets: [
    .target(
      name: "lib",
      dependencies: [
        .product(name: "v1", package: "api"),
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
    .executableTarget(
      name: "server",
      dependencies: [
        .product(name: "v1", package: "api"),
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
    .executableTarget(
      name: "controller",
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
    .executableTarget(
      name: "web",
      dependencies: [
        .product(name: "v1", package: "api"),
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
  ]
)
