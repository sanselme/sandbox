package main

import "fmt"

type Id int

func Has[T ~int](list []T, value T) bool {
	for _, v := range list {
		if v == value {
			return true
		}
	}
	return false
}

func main() {
	fmt.Printf("Has 2 %v\n", Has([]Id{1, 2}, 2))
}
