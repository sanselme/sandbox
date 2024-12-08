// SPDX-License-Identifier: GPL-3.0
//
//  form.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

struct Form: StaticPage {
  var title = "Form"

  func body(context: PublishingContext) async -> [BlockElement] {
    Text("Welcome to hello web form!").font(.title1)
  }
}
