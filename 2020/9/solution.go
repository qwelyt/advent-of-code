package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
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

func PossibleValues(numbers []int) map[int]bool {
	m := make(map[int]bool)
	for _, i := range numbers {
		for _, j := range numbers {
			m[i+j] = true
		}
	}
	return m
}

func FindInvalidXMAS(input []int, preamble int) (int, int, []int) {
	var position, value int

	previous := input[:preamble]

	for i := preamble + 1; i < len(input); i++ {
		number := input[i]
		possibleValues := PossibleValues(previous)
		if !possibleValues[number] {
			return number, i, previous
		}
		previous = nil
		previous = input[i-preamble : i+1]

	}

	return value, position, previous
}

func Sum(numbers []int) int {
	var sum int
	for _, i := range numbers {
		sum += i
	}
	return sum
}

func SumToNumber(numbers []int, goal int) []int {
	var list []int
	for i := 0; i < len(numbers); i++ {
		for j := 0; j < len(numbers); j++ {
			if i >= j {
				continue
			}
			var tmp = numbers[i:j]
			sum := Sum(tmp)
			if sum == goal {
				return tmp
			}
		}
	}

	return list
}

func LowHigh(numbers []int) (int, int) {
	var low, high = 1000000000000, 0
	for _, v := range numbers {
		if v < low {
			low = v
		} else if v > high {
			high = v
		}
	}
	return low, high
}

func main() {
	input := ReadFile("input.txt")
	partAValue, partAIndex, _ := FindInvalidXMAS(input, 25)
	fmt.Printf("=== Part A ===\nInvalidXMAS: %d,%d\n", partAValue, partAIndex)

	numberRange := SumToNumber(input[:partAIndex], partAValue)
	var lowest, highest = LowHigh(numberRange)
	fmt.Printf("=== Part B ===\nHigest: %d, Lowest: %d, Added: %d\n%v\n", highest, lowest, highest+lowest, numberRange)
}
