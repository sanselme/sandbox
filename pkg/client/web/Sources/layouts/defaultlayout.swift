// SPDX-License-Identifier: GPL-3.0
//
//  defaultlayout.swift
//  helloweb
//
//  Created by Schubert Anselme on 2024-11-11.
//

import Foundation
import Ignite

struct DefaultLayout: ContentPage {
  func body(content: Content, context: PublishingContext) -> [any BlockElement] {
    if let image = content.image {
      Image(image, description: content.imageDescription).resizable()
    }

    Text(content.title).font(.title1)

    if content.hasTags {
      Text { content.tagLinks(in: context) }.font(.title3)
    }

    Text(content.body)
  }
}
