// SPDX-License-Identifier: GPL-3.0

package main

import (
	apiv1 "api/v1"

	"context"
	"log"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main()  {
  // create a client connection to the gRPC server
  // this is where the gRPC-Gatewy proxies requests
  conn, err := grpc.NewClient(
    "0.0.0.0:8080",
    grpc.WithTransportCredentials(insecure.NewCredentials()),
  )
  if err != nil {
    log.Fatalln("failed to dial server:", err)
  }

  mux := runtime.NewServeMux()
  err = apiv1.RegisterGreeterHandler(context.Background(), mux, conn)
  if err != nil {
    log.Fatalln("failed to register gateway:", err)
  }

  server := &http.Server{
    Addr: ":80",
    Handler: mux,
  }

  log.Println("serving grpc-gateway on http://0.0.0.0:80")
  log.Fatalln(server.ListenAndServe())
}
