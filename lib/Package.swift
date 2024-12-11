// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "lib",
  products: [
    .library(name: "utils", targets: ["utils"])
  ],
  dependencies: [],
  targets: [
    .target(
      name: "utils",
      dependencies: [],
      path: "./utils"
    )
  ]
)
