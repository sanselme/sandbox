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
  "google.golang.org/grpc"
  v1 "greeter/api/v1"
  "log"
)

func getGreeting(name, coutryCode string, client v1.GreeterClient) {
  log.Println("Creating greeting...")

  response, err := client.Greet(context.Background(), &v1.GreetRequest{
    CountryCode: coutryCode,
    UserName: name,
  })
  if err != nil {
    log.Println("Error:", err)
    panic(err)
  }

  log.Println(response.Result)
}

func main() {
  cc, err := grpc.Dial("localhost:8080", grpc.WithInsecure())
  if err != nil {
    panic(err)
  }

  defer cc.Close()

  c := v1.NewGreeterClient(cc)
  getGreeting("Jack", "us", c)
  getGreeting("Jose", "mx", c)
}
