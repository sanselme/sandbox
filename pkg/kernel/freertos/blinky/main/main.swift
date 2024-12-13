// SPDX-License-Identifier: GPL-3.0
//
//  main.swift
//  helloesp
//
//  Created by Schubert Anselme on 2024-11-11.
//

// The code will blink an LED on GPIO8. To change the pin, modify LED(gpioPin: 8)
@_cdecl("app_main")
func app_main() {
  print("Hello from Swift on ESP32-C6!")

  var ledValue: Bool = false
  let blinkDelayMS: UInt32 = 500
  let led = LED(gpioPin: 8)

  while true {
    led.setLED(value: ledValue)
    ledValue.toggle()  // Toggle the boolean value
    vTaskDelay(blinkDelayMS / (1000 / UInt32(configTICK_RATE_HZ)))
  }
}