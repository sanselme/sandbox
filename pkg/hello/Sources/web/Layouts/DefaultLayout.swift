// SPDX-License-Identifier: GPL-3.0

struct DefaultLayout: ContentPage {
  func body(content: Content, context: PublishingContext) -> [any BlockElement] {
    Text(content.title).font(.title1)

    if let image = content.image {
      Image(image, description: content.imageDescription)
        .resizable()
        .cornerRadius(20)
        .frame(maxHeight: 300)
        .horizontalAlignment(.center)
    }

    if content.hasTag {
      Group {
        Text("Tagged with: \(content.tags.joined(separator: ", "))")
        Text(
          "\(content.estimatedWordCount) words; \(content.estimatedReadingMinutes) minutes to read."
        )
      }
    }

    Text(content.body)
  }
}
