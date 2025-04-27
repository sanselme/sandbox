// SPDX-License-Identifier: GPL-3.0

import Foundation
import Ignite

/// Displays a global "Social Footer", with each social icon linking to an
/// external site in a new browser tab, demonstrating how to create reusable
/// components with builtIn icons, external links and custom attributes.
public struct Footer: Component {
  let icons = [Image(systemName: "githhub")]

  let urlStrings = ["https://github.com/sanselme"]

  public func body(context: PublishingContext) -> [any PageElement] {
    Text {
      for (icon, urlString) in zip(icons, urlStrings) {
        Link(icon, target: urlString)
          .margin(.trailing, 20)
          .role(.secondary)
          .target(.blank)
          .relationship(.noOpener, .noReferrer)
      }
    }
    .font(.title2)
    .horizontalAlignment(.center)
    .margin(.top, .extraLarge)
  }
}
