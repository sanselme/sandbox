// SPDX-License-Identifier: GPL-3.0
//
//  web.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

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
