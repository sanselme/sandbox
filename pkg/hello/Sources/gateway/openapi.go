// SPDX-License-Identifier: GPL-3.0

package main

import (
	"fmt"
	"io/fs"
	"mime"
	"net/http"
	"openapi"
)

// Adapted from https://github.com/philips/grpc-gateway-example/blob/a269bcb5931ca92be0ceae6130ac27ae89582ecc/cmd/serve.go#L63
func getOpenAPIHandler() http.Handler {
  mime.AddExtensionType(".svg", "image/svg+xml")

  apifs, err := fs.Sub(openapi.OpenAPI, ".")
  if err != nil {
    panic(fmt.Sprintf("could not create sub filesystem: %v", err.Error()))
  }

  return http.FileServer(http.FS(apifs))
}
