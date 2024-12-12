// SPDX-License-Identifier: GPL-3.0
//
//  home.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

struct Home: StaticLayout {
  var title = "Home"

  var body: some HTML {
    Text("Welcome to hello web!").font(.title1)
  }
}
