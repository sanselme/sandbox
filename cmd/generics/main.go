package main

import "fmt"

func PrintThings[A, B any, C ~int](a1, a2 A, b B, c C) {
	fmt.Printf("%v %v %v %v\n", a1, a2, b, c)
}

func main() {
	PrintThings(1, 2, "three", 4)
}
