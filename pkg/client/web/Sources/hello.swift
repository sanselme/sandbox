// SPDX-License-Identifier: GPL-3.0
//
//  hello.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

struct Hello: Site {
  var name = "hello web"
  var titleSuffix = " - hello"
  var url = URL(filePath: "http://hello.sandbox.kube.local")
  var builtInIconsEnabled = true
  var author = "Schubert Anselme"

  // note: entrypoint
  var homePage = Home()
  var layout = EmptyLayout()
}
