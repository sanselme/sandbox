// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "sandbox",
  dependencies: [
    .package(path: "pkg/api"),
    .package(path: "pkg/hellod"),
  ]
)
