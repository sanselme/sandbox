// SPDX-License-Identifier: GPL-3.0

package main

import (
	"context"
	"fmt"
	"log"

	cloudevents "github.com/cloudevents/sdk-go/v2"
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

// handle shared the logic for producing the Response event from the Request.
// todo: send to hellod
func handle(req Request) Response {
	return Response{Message: fmt.Sprintf("Hello, %s", req.Name)}
}
