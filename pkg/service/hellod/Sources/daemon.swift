// SPDX-License-Identifier: GPL-3.0
//
//  daemon.swift
//  hellod
//
//  Created by Schubert Anselme on 2024-11-11.
//

import ArgumentParser
import GRPCCore
import GRPCNIOTransportHTTP2

@main
struct HelloDaemon: AsyncParsableCommand {
  static let configuration = CommandConfiguration(abstract: "Starts a greeter server.")

  @Option(help: "The port to listen on")
  var port: Int = 8080

  func run() async throws {
    let server = GRPCServer(
      transport: .http2NIOPosix(
        address: .ipv4(host: "127.0.0.1", port: self.port),
        transportSecurity: .plaintext
      ),
      services: [Greeter()]
    )

    try await withThrowingDiscardingTaskGroup { group in
      group.addTask { try await server.serve() }
      if let address = try await server.listeningAddress {
        print("Greeter listening on \(address)")
      }
    }
  }
}
