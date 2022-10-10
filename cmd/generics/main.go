package main

func Compare[T int | bool](a, b T) int {
  // a == b -> 0
  if a == b {
    return 0
  }

  // a < b -> -1
  if a < b {
    return -1
  }

  return 1
}

func main() {
}
