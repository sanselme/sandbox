// SPDX-License-Identifier: GPL-3.0
//
//  navbar.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

/// An example navigation bar, demonstrating how to create reusable components.
struct NavBar: Component {
  func body(context: PublishingContext) -> [any PageElement] {
    NavigationBar(
      logo: Image("/images/logo.svg", description: "hello").frame(
        width: "min(60vw, 300px)", height: "100%")
    ) {
      Link("hello", target: "https://github.com/sanselme/sandbox.git")
    }
    .navigationItemAlignment(.trailing)
    .navigationBarStyle(.dark)
    .background(.firebrick)
    .position(.fixedTop)
  }
}
