package main

import "fmt"

type Equalizer[T any] interface {
	Equal(other T) bool
}

type Id int

func (i Id) Equal(other Id) bool {
	return i == other
}

func Has[T Equalizer[T]](list []T, value T) bool {
	for _, v := range list {
		if value.Equal(v) {
			return true
		}
	}
	return false
}

func main() {
	fmt.Printf("Has 2 %v\n", Has([]Id{1, 2}, 2))
}
