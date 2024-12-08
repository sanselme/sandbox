// SPDX-License-Identifier: GPL-3.0
//
//  default.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

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
