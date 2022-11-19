/*
Copyright © 2022 Schubert Anselme <schubert@anselm.es>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

package main

import (
	"context"
	"fmt"
	"log"
	"net"

	v1 "github.com/sanselme/sandbox/api/v1"

	"google.golang.org/grpc"
)

type server struct{}

func (s server) Greet(ctx context.Context, request *v1.GreetRequest) (*v1.GreetResponse, error) {
  log.Println("Username:", request.UserName)
  log.Println("Country Code:", request.CountryCode)

  var greeting string

  switch request.CountryCode {
  case "us":
    greeting = fmt.Sprintf("Hello %s", request.UserName)
  case "mx":
    greeting = fmt.Sprintf("Hola %s", request.UserName)
  default:
    greeting = fmt.Sprintf("Hello/Hola %s", request.UserName)
  }

  return &v1.GreetResponse{Result: greeting}, nil
}

func main() {
  listener, err := net.Listen("tcp", "localhost:8080")
  if err != nil {
    panic(err)
  }

  fmt.Println("starting server...")

  s := grpc.NewServer()
  v1.RegisterGreeterServer(s, server{})

  if err := s.Serve(listener); err != nil {
    panic(err)
  }
}
