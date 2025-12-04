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

func FindPath(treeMap []string, open string, tree string, right int, down int) Path {
	x := 0
	y := 0

	var steps, opens, trees int

	for checkY := y + down; checkY < len(treeMap); checkY += down {
		checkX := x + right
		row := treeMap[checkY]
		if checkX >= len(row) {
			checkX = checkX - len(row)
			x -= len(row)
		}
		pos := string(row[checkX])
		if pos == tree {
			trees++
		} else {
			opens++
		}
		steps++
		x += right
		y += down
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
