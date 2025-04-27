// SPDX-License-Identifier: GPL-3.0

import Foundation
import Ignite

struct Form: StaticPage {
  var title = "Form"

  func body(context: PublishingContext) async -> [BlockElement] {
    Text("Welcome to hello web form!").font(.title1)
  }
}
