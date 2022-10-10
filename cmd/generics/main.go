package main

import (
	"fmt"
	"math"
)

type Currency interface {
	~int | ~int64
	ISO4127Code() string
	Decimal() int
}

type CAD int64

func (c CAD) ISO4127Code() string {
	return "CAD"
}

func (c CAD) Decimal() int {
	return 2
}

func PrintBalance[T Currency](b T) {
	balance := float64(b) / math.Pow10(b.Decimal())
	fmt.Printf("%.*f %s\n", b.Decimal(), balance, b.ISO4127Code())
}

func main() {
	PrintBalance(CAD(250))
}
