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
	fmt.Printf("Has 2 %v\n", Has([]int{1, 2}, 2))
}
