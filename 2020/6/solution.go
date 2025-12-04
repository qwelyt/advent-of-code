package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func ReadFile(filePath string) []string {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values []string
	for scanner.Scan() {
		values = append(values, scanner.Text())
	}
	return values
}

func reduceToArray(group []string) []string {
	var tmp []string
	for _, v := range group {
		chars := strings.Split(v, "")
		tmp = append(tmp, chars...)
	}
	return tmp
}

func CountGroupAnyYes(group []string) int {
	var m = make(map[string]bool)

	for _, v := range reduceToArray(group) {
		m[v] = true
	}

	return len(m)
}

func CountGroupAllYes(group []string) int {
	var m = make(map[string]int)
	var num int

	for _, v := range reduceToArray(group) {
		m[v] = m[v] + 1
	}

	for _, v := range m {
		if v == len(group) {
			num++
		}
	}
	return num
}

func CountYesPerGroup(input []string, countFn func([]string) int) []int {
	var yes []int
	var group []string

	for _, row := range input {
		if len(row) == 0 {
			y := countFn(group)
			yes = append(yes, y)
			group = nil
		} else {
			group = append(group, row)
		}
	}
	if len(group) > 0 {
		y := countFn(group)
		yes = append(yes, y)
		group = nil
	}

	return yes
}

func Count(input []string, countFn func([]string) int) int {
	var result int
	ypg := CountYesPerGroup(input, countFn)
	for _, y := range ypg {
		result = result + y
	}
	return result
}

func main() {
	// input := ReadFile("example.txt")
	input := ReadFile("input.txt")
	partA := Count(input, CountGroupAnyYes)
	fmt.Printf("=== Part A ===\nSum yeses: %d\n", partA)
	partB := Count(input, CountGroupAllYes)
	fmt.Printf("=== Part B ===\nSum yeses: %d\n", partB)
}
