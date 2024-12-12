// SPDX-License-Identifier: GPL-3.0
//
//  web.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import ArgumentParser
import Foundation
import Ignite

@main
struct Web: ParsableCommand {
  static let configuration = CommandConfiguration(abstract: "Hello web client")

  static func main() async {
    let web = Hello()

    do {
      try await web.publish()
    } catch {
      print(error.localizedDescription)
    }
  }
}
