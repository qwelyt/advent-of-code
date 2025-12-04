package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

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
		t := scanner.Text()
		i, err := strconv.Atoi(t)
		if err != nil {
			log.Fatal(err)
		}
		values = append(values, i)
	}
	sort.Ints(values)
	return values
}

func JoltageJumps(numbers []int) (int, int) {
	var one, three int
	for i := 0; i < len(numbers); i++ {
		if i+1 == len(numbers) {
			continue
		}
		diff := numbers[i+1] - numbers[i]
		switch diff {
		case 1:
			one++
		case 3:
			three++
		}
	}
	return one + 1, three + 1
}

func ToSet(n []int) map[int]bool {
	var m = make(map[int]bool)
	for _, v := range n {
		m[v] = true
	}
	return m
}

func DA(in []int) int {
	var numbers = []int{0}
	numbers = append(numbers, in...)
	numbers = append(numbers, numbers[len(numbers)-1]+3)
	stepCounter := make(map[int]int)
	stepCounter[0] = 1

	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers) && numbers[j] <= numbers[i]+3; j++ {
			fmt.Printf("i %d, j, %d ::: %d ::: %d + %d = %d\n", i, j, numbers[j], stepCounter[i], stepCounter[j], stepCounter[j]+stepCounter[i])
			stepCounter[j] = stepCounter[j] + stepCounter[i]
		}
	}
	fmt.Println(stepCounter)
	return stepCounter[len(numbers)-1]
}

func main() {
	input := ReadFile("input.txt")
	one, three := JoltageJumps(input)
	fmt.Printf("=== Part A ===\nOnes: %d, Threes: %d, Multiplied: %d\n", one, three, one*three)
	val := DA(input)
	fmt.Printf("=== Part B ===\nPaths: %d\n", val)
}
