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

func main() {
	fmt.Printf("Has: a %v\n", Has([]string{"a", "b"}, "a"))
	fmt.Printf("Has: c %v\n", Has([]string{"a", "b"}, "c"))
}
