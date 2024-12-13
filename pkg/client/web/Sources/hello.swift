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
  var url = URL(string: ProcessInfo.processInfo.environment["URL"] ?? "http://localhost:8080")!
  var author = "Schubert Anselme"

  // note: configuration
  var builtInIconsEnabled = true
  var syntaxHighlighters = [SyntaxHighlighter.swift, .python, .ruby]
  var feedConfiguration = FeedConfiguration(
    mode: .full, contentCount: 20, image: .init(url: "/images/icon32.png", width: 32, height: 32))
  var robotsConfiguration = Robots()

  // note: homepage
  var homePage = Home()

  // todo: pages

  // note: theme & layout
  var theme = DefaultTheme()
  var layouts: [any ContentPage] { DefaultLayout() }
}
