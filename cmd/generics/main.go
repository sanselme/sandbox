package main

import "fmt"

func Has[T comparable](list []T, value T) bool {
  for _, v := range list {
    if v == value {
      return true
    }
  }
  return false
}

func NewEmptyList[T any]() []T {
  return make([]T, 0)
}

func main() {
  fmt.Printf("Has: a %v\n", Has([]string{"a", "b"}, "a"))
  fmt.Printf("Has: c %v\n", Has([]string{"a", "b"}, "c"))
  fmt.Printf("Has: 2 %v\n", Has([]int{1, 2, 3}, 2))

  fmt.Printf("MyList: %v\n", NewEmptyList[int]())
}
