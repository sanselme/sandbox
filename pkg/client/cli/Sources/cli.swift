// SPDX-License-Identifier: GPL-3.0
//
//  cli.swift
//  hello
//
//  Created by Schubert Anselme on 2024-11-11.
//

import ArgumentParser
import AsyncHTTPClient
import Foundation
import NIOCore
import v1

@main
struct Greet: AsyncParsableCommand {
  static let configuration = CommandConfiguration(abstract: "Send a request to the greeter server")

  @Option(help: "The address to connect to")
  var address: String = "127.0.0.1"

  @Option(help: "The port to listen on")
  var port: Int = 80

  @Option(help: "The person to greet")
  var name: String = ""

  func run() async throws {
    let client = HTTPClient(eventLoopGroupProvider: .singleton)

    do {
      var request = HTTPClientRequest(
        url: "http://\(self.address):\(self.port)/api/v1/sandbox/hello")
      request.method = .POST
      request.headers.add(name: "User-Agent", value: "Swift AsyncHTTPClient")
      request.headers.add(name: "Content-Type", value: "application/json")

      var greet = v1.Api_V1_HelloRequest()
      greet.name = self.name

      let data: Data = try greet.jsonUTF8Data()
      request.body = .bytes(data)

      let response = try await client.execute(request, timeout: .seconds(5))
      if response.status == .ok {
        if let contentType = response.headers.first(name: "Content-Type"),
          contentType.contains("application/json")
        {
          let buffer = try await response.body.collect(upTo: 1024 * 1024)
          let data = buffer.withUnsafeReadableBytes { Data($0) }
          let output = try v1.Api_V1_HelloReply(jsonUTF8Data: data)
          print(output.message)
        }
      } else {
        print("invalid status code: \(response.status)")
      }
    } catch {
      print("\(error)")
    }

    try await client.shutdown()
  }
}
