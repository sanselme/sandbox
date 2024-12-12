// SPDX-License-Identifier: GPL-3.0

package main

import (
	apiv1 "api/v1"
	"context"
	"fmt"
	"log"
	"time"

	cloudevents "github.com/cloudevents/sdk-go/v2"
	"github.com/kelseyhightower/envconfig"
	"github.com/spf13/cobra"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
  grpcPort int
  grpcAddress string

  rootCmd = &cobra.Command {
    Use: "helloce",
    Short: "CloudEvents service",
    Run: func(cmd *cobra.Command, args []string) {
      client, err := cloudevents.NewDefaultClient()
      if err != nil {
        log.Fatal(err.Error())
      }

      r := Receiver{client: client}
      if err := envconfig.Process("", &r); err != nil {
        log.Fatal(err.Error())
      }

      // Depending on whether targeting data has been supplied,
      // we will either reply with our response or send it on to
      // an event sink.
      var receiver interface{} // the SDK reflects on the signature.
      if r.Target == "" {
        receiver = r.ReceiveAndReply
      } else {
        receiver = r.ReceiveAndSend
      }

      if err := client.StartReceiver(context.Background(), receiver); err != nil {
        log.Fatal(err)
      }
    },
  }
)

type Receiver struct {
	client cloudevents.Client

	// If the K_SINK environment variable is set, then events are sent there,
	// otherwise we simply reply to the inbound request.
	Target string `envconfig:"K_SINK"`
}

// Request is the structure of the event we expect to receive.
type Request struct {
	Name string `json:"name"`
}

// Response is the structure of the event we send in response to requests.
type Response struct {
	Message string `json:"message,omitempty"`
}

// ReceiveAndSend is invoked whenever we receive an event.
func (recv *Receiver) ReceiveAndSend(ctx context.Context, event cloudevents.Event) cloudevents.Result {
	req := Request{}
	if err := event.DataAs(&req); err != nil {
		return cloudevents.NewHTTPResult(400, "failed to convert data: %s", err)
	}
	log.Printf("Got an event from: %q", req.Name)

	resp := handle(req)
	log.Printf("Sending event: %q", resp.Message)

	r := cloudevents.NewEvent(cloudevents.VersionV1)
	r.SetType("es.anselm.sandbox.hello")
	r.SetSource("https://github.com/sanselme/sandbox")
	if err := r.SetData("application/json", resp); err != nil {
		return cloudevents.NewHTTPResult(500, "failed to set response data: %s", err)
	}

	ctx = cloudevents.ContextWithTarget(ctx, recv.Target)
	return recv.client.Send(ctx, r)
}

// ReceiveAndReply is invoked whenever we receive an event.
func (recv *Receiver) ReceiveAndReply(ctx context.Context, event cloudevents.Event) (*cloudevents.Event, cloudevents.Result) {
	req := Request{}
	if err := event.DataAs(&req); err != nil {
		return nil, cloudevents.NewHTTPResult(400, "failed to convert data: %s", err)
	}
	log.Printf("Got an event from: %q", req.Name)

	resp := handle(req)
	log.Printf("Replying with event: %q", resp.Message)

	r := cloudevents.NewEvent(cloudevents.VersionV1)
	r.SetType("es.anselm.sandbox.hello")
	r.SetSource("https://github.com/sanselme/sandbox")
	if err := r.SetData("application/json", resp); err != nil {
		return nil, cloudevents.NewHTTPResult(500, "failed to set response data: %s", err)
	}

	return &r, nil
}

// Execute runs the root command.
func Execute() error {
  return rootCmd.Execute()
}

// handle shared the logic for producing the Response event from the Request.
func handle(req Request) Response {
  conn, err := grpc.NewClient(fmt.Sprintf("%s:%d", grpcAddress, grpcPort), grpc.WithTransportCredentials(insecure.NewCredentials()))
  if err != nil {
    log.Fatalln("failed to dial server:", err)
  }
  defer conn.Close()

  client := apiv1.NewGreeterClient(conn)
  ctx, cancel := context.WithTimeout(context.Background(), time.Second)
  defer cancel()

  r, err := client.SayHello(ctx, &apiv1.HelloRequest{Name: req.Name})
  if err != nil {
    log.Fatalln("failed to send request:", err)
  }

	return Response{Message: r.Message}
}

func init() {
  rootCmd.PersistentFlags().IntVarP(&grpcPort, "grpc-port", "", 8080, "port of the gRPC server")
  rootCmd.PersistentFlags().StringVarP(&grpcAddress, "grpc-address", "", "127.0.0.1", "address of the gRPC server")
}
