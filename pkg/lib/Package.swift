// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "lib",
  products: [
    .library(name: "libhello", targets: ["hello"])
  ],
  dependencies: [
    .package(path: "../api")
    // .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0"),
  ],
  targets: [
    .target(
      name: "hello",
      dependencies: [
        .product(name: "v1", package: "api")
        // .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ],
      path: "./hello"
    )
  ]
)
