// SPDX-License-Identifier: GPL-3.0
//
//  web.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

@main
struct Web {
  static func main() async {
    let web = Hello()

    do {
      try await web.publish()
    } catch {
      print(error.localizedDescription)
    }
  }
}
