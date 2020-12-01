package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Pair struct {
	left, right int
}

func ReadFile(filePath string) []int {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values []int
	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		values = append(values, value)
	}
	return values
}

func SumToTarget(values []int, target int) Pair {
	for i := 0; i < len(values); i++ {
		for j := 0; j < len(values); j++ {
			if i == j {
				continue
			}
			if values[i]+values[j] == target {
				return Pair{values[i], values[j]}
			}
		}
	}
	return Pair{0, 0}
}

func Multiply(a int, b int) int {
	return a * b
}

func main() {
	values := ReadFile("../input.txt")
	//values := readFile("./example.txt")
	pair := SumToTarget(values, 2020)
	multiply := Multiply(pair.left, pair.right)
	fmt.Println(multiply)
}
