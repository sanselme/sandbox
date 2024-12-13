// SPDX-License-Identifier: GPL-3.0
//
//  robots.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

/// An example custom robots.txt configuration file, blocking certain paths
/// from Google and Bing, and everything from ChatGPT.
struct Robots: RobotsConfiguration {
  var disallowRules: [DisallowRule]

  init() {
    let paths = [
      "/admin",
      "/settings",
    ]

    disallowRules = [
      DisallowRule(robot: .google, paths: paths),
      DisallowRule(robot: .bing, paths: paths),
      DisallowRule(robot: .chatGPT),
    ]
  }
}
