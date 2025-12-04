package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Path struct {
	steps, open, trees int
}

type Slope struct {
	right, down int
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

func FindPath(rows []string, open string, tree string, right int, down int) Path {
	var steps, opens, trees int

	for y, x := down, 0; y < len(rows); y += down {
		row := rows[y]
		x = (x + right) % len(row)
		sign := string(row[x])
		switch sign {
		case tree:
			trees++
		case open:
			opens++
		}
		steps++
	}
	return Path{steps, opens, trees}
}

func PartA(input []string) Path {
	return FindPath(input, ".", "#", 3, 1)
}

func PartB(input []string) int {
	slopes := []Slope{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2}}

	var paths []Path
	for _, slope := range slopes {
		paths = append(paths, FindPath(input, ".", "#", slope.right, slope.down))
	}

	multipliedTrees := 1
	for _, path := range paths {
		multipliedTrees = path.trees * multipliedTrees
	}

	return multipliedTrees
}

func main() {
	input := ReadFile("input.txt")
	// input := ReadFile("example.txt")
	partA := PartA(input)
	partB := PartB(input)
	fmt.Printf("=== Part A ===\nSteps: %d, Open: %d, Trees: %d\n", partA.steps, partA.open, partA.trees)
	fmt.Printf("=== Part B ===\nTrees: %d\n", partB)
}
