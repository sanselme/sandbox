// SPDX-License-Identifier: GPL-3.0

package main

import (
	apiv1 "api/v1"
	"context"
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"github.com/spf13/cobra"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
  grpcPort int
  gwPort int
  rootCmd = &cobra.Command {
    Use: "hellogw",
    Short: "gRPC-Gateway proxy for the gRPC Greeter service",
    Run: func(cmd *cobra.Command, args []string) {
      // create a client connection to the gRPC server
      // this is where the gRPC-Gatewy proxies requests
      conn, err := grpc.NewClient(
        fmt.Sprintf("0.0.0.0:%d", grpcPort),
        grpc.WithTransportCredentials(insecure.NewCredentials()),
      )
      if err != nil {
        log.Fatalln("failed to dial server:", err)
      }
      defer conn.Close()

      mux := runtime.NewServeMux()
      err = apiv1.RegisterGreeterHandler(context.Background(), mux, conn)
      if err != nil {
        log.Fatalln("failed to register gateway:", err)
      }

      oa := getOpenAPIHandler()

      server := &http.Server{
        Addr: fmt.Sprintf(":%d", gwPort),
        Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
          if strings.HasPrefix(r.URL.Path, "/api") {
            mux.ServeHTTP(w, r)
            return
          }
          oa.ServeHTTP(w, r)
        }),
      }

      log.Printf("serving grpc-gateway on http://0.0.0.0:%d\n", gwPort)
      log.Fatalln(server.ListenAndServe())
    },
  }
)

func Execute() error {
  return rootCmd.Execute()
}

// todo: add tls
func init() {
  rootCmd.PersistentFlags().IntVarP(&grpcPort, "grpc", "", 8080, "port of the gRPC server")
  rootCmd.PersistentFlags().IntVarP(&gwPort, "gw", "", 80, "port of the gRPC-Gateway server")
}
