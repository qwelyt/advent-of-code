package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Path struct {
	colour string
	cost   int
}

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

func BagsInside(bagLine string) map[string]int {
	var m = make(map[string]int)

	bags := strings.Split(strings.Replace(bagLine, ".", "", -1), ", ")
	for _, bag := range bags {
		b := strings.Split(bag, " ")
		n, err := strconv.Atoi(bag[0:len(b[0])])
		if err != nil {
			fmt.Printf("Could not parse %v\n", bag)
		}
		k := bag[len(b[0])+1 : len(bag)]
		m[k] = n
	}
	return m
}

func BagStructure(input []string) map[string]map[string]int {
	var m = make(map[string]map[string]int)

	for _, row := range input {
		cleanedRow := strings.Replace(strings.Replace(row, " bags", "", -1), " bag", "", -1)
		rs := strings.Split(cleanedRow, " contain ")
		k := rs[0]
		if rs[1] == "no other" || rs[1] == "no other." {
			m[k] = make(map[string]int)
		} else {
			m[k] = BagsInside(rs[1])
		}
	}
	return m
}

func traverse(curr string, rules map[string]map[string]int, find string) (bool, []string) {
	var path []string
	lookAt := rules[curr]
	if _, ok := lookAt[find]; ok {
		path = append(path, curr)
		return true, path
	}
	for k, _ := range lookAt {
		if k == find {
			continue
		}
		present, p := traverse(k, rules, find)
		path = append(path, p...)
		if present {
			return true, path
		}

	}
	return false, path
}

func PathTo(rules map[string]map[string]int, find string) [][]string {
	var paths [][]string
	for k, _ := range rules {
		present, path := traverse(k, rules, find)
		if present {
			paths = append(paths, path)
		}
	}
	return paths
}

func counter(rules map[string]map[string]int, find string) int {
	topBags := rules[find]
	count := 1
	for k, v := range topBags {
		count += counter(rules, k) * v
	}
	return count
}

func CountBags(rules map[string]map[string]int, find string) int {
	count := counter(rules, find) - 1
	return count
}

func main() {
	input := ReadFile("input.txt")
	bagStructure := BagStructure(input)
	partA := PathTo(bagStructure, "shiny gold")
	fmt.Printf("=== Part A ===\nPossible structures: %d\n", len(partA))

	partB := CountBags(bagStructure, "shiny gold")
	fmt.Printf("=== Part B ===\nBags needed: %d\n", partB)

}
