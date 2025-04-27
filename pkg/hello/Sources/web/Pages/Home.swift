// SPDX-License-Identifier: GPL-3.0

import Foundation
import Ignite

struct Home: StaticPage {
  var title = "Home"

  func body(context: PublishingContext) async -> [BlockElement] {
    Text("Welcome to hello web!").font(.title1)
  }
}
