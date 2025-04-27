// SPDX-License-Identifier: GPL-3.0

import GRPCCore
import api

struct Greeter: Api_V1_Greeter.ServiceProtocol {
  func sayHello(
    request: ServerRequest<Api_V1_HelloRequest>,
    context: ServerContext
  ) async throws -> ServerResponse<Api_V1_HelloReply> {
    var reply = Api_V1_HelloReply()
    let recipient = request.message.name.isEmpty ? "stranger" : request.message.name
    reply.message = "Hello, \(recipient)"
    return ServerResponse(message: reply)
  }
}
