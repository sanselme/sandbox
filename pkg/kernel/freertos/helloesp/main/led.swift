// SPDX-License-Identifier: GPL-3.0
//
//  led.swift
//  helloesp
//
//  Created by Schubert Anselme on 2024-11-11.
//

// A simple "overlay" to provide nicer APIs in Swift
struct LED {
  var ledPin: gpio_num_t

  init(gpioPin: Int) {
    ledPin = gpio_num_t(Int32(gpioPin))

    guard gpio_reset_pin(ledPin) == ESP_OK else { fatalError("cannot reset led") }
    guard gpio_set_direction(ledPin, GPIO_MODE_OUTPUT) == ESP_OK else {
      fatalError("cannot reset led")
    }
  }

  func setLED(value: Bool) {
    let level: UInt32 = value ? 1 : 0
    gpio_set_level(ledPin, level)
  }
}
