// SPDX-License-Identifier: GPL-3.0

import Foundation
import Ignite

/// An example custom robots.txt configuration file, blocking certain paths
/// from Google and Bing, and everything from ChatGPT.
struct Robot: RobotsConfiguration {
  var disallowRules: [DisallowRule]

  init() {
    // todo: add paths
    // let paths = []

    disallowRules = [
      DisallowRule(robot: .google, paths: paths),
      DisallowRule(robot: .bing, paths: paths),
      DisallowRule(robot: .chatGPT),
    ]
  }
}
