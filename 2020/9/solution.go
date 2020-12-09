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
	return values
}

func Find(numbers []int, goal int) bool {
	var m = make(map[int]bool)
	for _, v := range numbers {
		m[v] = true
	}
	for _, v := range numbers {
		if m[goal-v] {
			return true
		}
	}
	return false
}

func FindInvalidXMAS(input []int, preamble int) (int, int, []int) {
	previous := input[:preamble]
	for i := preamble; i < len(input); i++ {
		number := input[i]
		if !Find(previous, number) {
			return number, i, previous
		}
		previous = nil
		previous = input[i-preamble : i+1]
	}

	return 0, 0, previous
}

func Sum(numbers []int) int {
	var sum int
	for _, i := range numbers {
		sum += i
	}
	return sum
}

func SumToNumber(numbers []int, goal int) ([]int, bool) {
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			var tmp = numbers[i:j]
			sum := Sum(tmp)
			if sum == goal {
				return tmp, true
			}
		}
	}
	return []int{}, false
}

func LowHigh(numbers []int) (int, int) {
	sort.Ints(numbers)
	return numbers[0], numbers[len(numbers)-1]
}

func actual() {
	input := ReadFile("input.txt")
	partAValue, partAIndex, _ := FindInvalidXMAS(input, 25)
	fmt.Printf("=== Part A ===\nInvalidXMAS: %d,%d\n", partAValue, partAIndex)

	numberRange, foundRange := SumToNumber(input[:partAIndex], partAValue)
	if !foundRange {
		log.Fatal("Could not sum to goal")
	}
	var lowest, highest = LowHigh(numberRange)
	fmt.Printf("=== Part B ===\nHighest: %d, Lowest: %d, Added: %d\n%v\n", highest, lowest, highest+lowest, numberRange)
}

func Time() {
	input := ReadFile("input.txt")
	partAValue, partAIndex, _ := FindInvalidXMAS(input, 25)
	if partAValue != 258585477 {
		log.Fatal("Could not find PartA answer")
	}

	numberRange, foundRange := SumToNumber(input[:partAIndex], partAValue)
	if !foundRange {
		log.Fatal("Could not sum to goal")
	}
	var lowest, highest = LowHigh(numberRange)
	if lowest+highest != 36981213 {
		log.Fatal("Could not calculate the correct PartB answer")
	}
}

func main() {
	actual()
	// time()
}
