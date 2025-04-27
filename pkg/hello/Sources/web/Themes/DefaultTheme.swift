// SPDX-License-Identifier: GPL-3.0

import Foundation
import Ignite

struct DefaultTheme: Theme {
  func render(page: Page, context: PublishingContext) async -> HTML {
    HTML {
      Head(for: page, in: context)
      Body {
        // todo: navbar
        page.body
        // todo: footer
        // Group {}
      }
      .padding(.vertical, 80)
      .class("container")
    }
  }
}
