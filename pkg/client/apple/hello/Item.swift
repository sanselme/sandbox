//
//  Item.swift
//  hello
//
//  Created by Schubert Anselme on 2024-12-05.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
