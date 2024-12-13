// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "helloweb",
  platforms: [.macOS("15.0")],
  products: [
    .executable(name: "helloweb", targets: ["webclient"])
  ],
  dependencies: [
    .package(url: "https://github.com/twostraws/Ignite.git", from: "0.2.2")
  ],
  targets: [
    .executableTarget(
      name: "webclient",
      dependencies: [
        .product(name: "Ignite", package: "Ignite")
      ]
    )
  ]
)
