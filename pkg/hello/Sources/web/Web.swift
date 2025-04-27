// SPDX-License-Identifier: GPL-3.0

import ArgumentParser
import Ignite

@main
struct Web: ParsableCommand {
  static let configuration = CommandConfiguration(abstract: "Hello web client")

  func run() async throws {
    let web = await Hello()

    do {
      try await web.publish()
    } catch {
      print(error.localizedDescription)
    }
  }
}
