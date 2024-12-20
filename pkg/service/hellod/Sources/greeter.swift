// SPDX-License-Identifier: GPL-3.0
//
//  greet.swift
//  hellod
//
//  Created by Schubert Anselme on 2024-11-11.
//

import GRPCCore
import GRPCNIOTransportHTTP2
import v1

struct Greeter: v1.Api_V1_Greeter.ServiceProtocol {
  func sayHello(
    request: ServerRequest<v1.Api_V1_HelloRequest>,
    context: ServerContext
  ) async throws -> ServerResponse<v1.Api_V1_HelloReply> {
    var reply = v1.Api_V1_HelloReply()
    let recipient = request.message.name.isEmpty ? "stranger" : request.message.name
    reply.message = "Hello, \(recipient)"
    return ServerResponse(message: reply)
  }
}
