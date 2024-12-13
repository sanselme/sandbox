// SPDX-License-Identifier: GPL-3.0
//
//  defaultlayout.swift
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
        NavBar()

        page.body

        Group {
          SocialFooter()
          IgniteFooter()
        }
      }
      .padding(.vertical, 80)
      .class("container")
    }
  }
}
