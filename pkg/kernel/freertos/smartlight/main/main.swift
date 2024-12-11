// SPDX-License-Identifier: GPL-3.0
//
//  main.swift
//  helloesp
//
//  Created by Schubert Anselme on 2024-11-11.
//

@_cdecl("app_main")
func app_main() {
  print("🏎️   Hello, Embedded Swift! (Smart Light)")

  var led = LED()

  // (1) Create a Matter root node
  let rootNode = Matter.Node()
  rootNode.identifyHandler = {
    print("identify")
  }

  // (2) Create a "light" endpoint
  let lightEndpoint = Matter.ExtendedColorLight(node: rootNode)
  lightEndpoint.eventHandler = { event in
    print("lightEndpoint.eventHandler:")
    print(event.attribute)
    print(event.value)

    switch event.attribute {
    case .onOff:
      led.enabled = (event.value == 1)

    case .levelControl:
      led.brightness = Int(Float(event.value) / 255.0 * 100.0)

    case .colorControl(.currentHue):
      let newHue = Int(Float(event.value) / 255.0 * 360.0)
      led.color = .hueSaturation(newHue, led.color.saturation)

    case .colorControl(.currentSaturation):
      let newSaturation = Int(Float(event.value) / 255.0 * 100.0)
      led.color = .hueSaturation(led.color.hue, newSaturation)

    case .colorControl(.colorTemperatureMireds):
      let kelvins = 1_000_000 / event.value
      led.color = .temperature(kelvins)

    default:
      break
    }
  }

  // (3) Add the endpoint to the node
  rootNode.addEndpoint(lightEndpoint)

  // (4) Provide the node to a Matter application and start it
  let app = Matter.Application()
  app.rootNode = rootNode
  app.start()

  // Keep local variables alive. Workaround for issue #10
  // https://github.com/apple/swift-matter-examples/issues/10
  while true {
    sleep(1)
  }
}
