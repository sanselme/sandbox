// SPDX-License-Identifier: GPL-3.0

package main

import (
	apiv1 "api/v1"
	"context"
	"fmt"
	"log"
	"time"

	"github.com/spf13/cobra"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
  port int
  name string
  rootCmd = &cobra.Command {
    Use: "hellocli",
    Short: "Send a request to the greeter server",
    Run: func(cmd *cobra.Command, args []string) {
      conn, err := grpc.NewClient(
        fmt.Sprintf("0.0.0.0:%d", port),
        grpc.WithTransportCredentials(insecure.NewCredentials()),
      )
      if err != nil {
        log.Fatalln("failed to dial server:", err)
      }
      defer conn.Close()

      client := apiv1.NewGreeterClient(conn)
      ctx, cancel := context.WithTimeout(context.Background(), time.Second)
      defer cancel()

      req, err := client.SayHello(ctx, &apiv1.HelloRequest{Name: name})
      if err != nil {
        log.Fatalln("failed to send request:", err)
      }

      log.Printf("%s", req.Message)
    },
  }
)

func Execute() error {
  return rootCmd.Execute()
}

func init() {
  rootCmd.PersistentFlags().IntVarP(&port, "port", "", 8080, "The port to listen on")
  rootCmd.PersistentFlags().StringVarP(&name, "name", "", "", "The person to greet")
}
