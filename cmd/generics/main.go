package main

import "fmt"

func Has[T any](list []T, value T, equal func(a, b T) bool) bool {
  for _, v := range list {
    if equal(v, value) {
      return true
    }
  }
  return false
}

func main() {
  equalInt := func(a, b int) bool { return a == b }

  fmt.Printf("Has 2 %v\n", Has([]int{1, 2}, 2, equalInt))
}
