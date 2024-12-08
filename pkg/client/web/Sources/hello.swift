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
  var url = URL(filePath: "http://web.hello.local")
  var builtInIconsEnabled = true
  var author = "Schubert Anselme"

  // fixme: extra config
  // note: configuration
  // var syntaxHighlighters = [SyntaxHighlighter.swift, .python, .ruby]
  // var feedConfiguration = FeedConfiguration(
  //   mode: .full, contentCount: 20,
  //   image: .init(url: "http://web.hello.local/images/icon32.png", width: 32, height: 32))
  var robotsConfiguration = Robot()

  // note: entrypoint
  var homePage = Home()
  var theme = DefaultTheme()

  // todo: add pages
  // note: pages
  // var pages: [any StaticPage] {
  //   Form()
  // }

  // todo: add layouts
  // note: layouts
  // var layouts: [any ContentPage] {
  //   DefaultLayout()
  // }
}
