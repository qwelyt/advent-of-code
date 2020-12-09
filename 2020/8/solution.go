package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Sequence struct {
	acc, lineChanged int
	steps            []int
	visited          map[int]bool
}

func (seq Sequence) String() string {
	return fmt.Sprintf("Acc: %d\nLinechange: %d\nSteps: %v\nVisited: %v", seq.acc, seq.lineChanged, seq.steps, seq.visited)
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

func BootLoop(input []string) (int, map[int]bool, []int, bool) {
	var visited = make(map[int]bool)
	var steps []int
	var acc int

	for i := 0; i < len(input); {
		steps = append(steps, i)
		if alreadyDone := visited[i]; alreadyDone {
			return acc, visited, steps, false
		}
		visited[i] = true
		row := strings.Split(input[i], " ")
		switch row[0] {
		case "nop":
			i++
		case "acc":
			value, err := strconv.Atoi(row[1])
			if err != nil {
				log.Fatal(err)
			}
			acc += value
			i++
		case "jmp":
			value, err := strconv.Atoi(row[1])
			if err != nil {
				log.Fatal(err)
			}
			i += value
		}
	}

	return acc, visited, steps, true
}

func BruteForceSequence(input []string) (int, int, []int) {
	var m []Sequence

	for i := 0; i < len(input); i++ {
		row := strings.Split(input[i], " ")
		op := row[0]
		if op == "jmp" || op == "nop" {
			switch op {
			case "jmp":
				op = "nop"
			case "nop":
				op = "jmp"
			}
			var newInput []string
			newInput = append(newInput, input[:i]...)
			newLine := op + " " + row[1]
			newInput = append(newInput, newLine)
			newInput = append(newInput, input[i+1:]...)
			a, visited, steps, reachedEnd := BootLoop(newInput)
			if reachedEnd {
				// We could just return here, but doing an exhaustive
				// search seems like a nice way to catch bugs.
				seq := Sequence{a, i, steps, visited}
				m = append(m, seq)
			}
		}
	}

	if len(m) != 1 {
		log.Fatalf("Did not find just one solution: %v", m)
	}

	seq := m[0]

	return seq.acc, seq.lineChanged, seq.steps
}

func main() {
	// input := ReadFile("example.txt")
	input := ReadFile("input.txt")
	acc, visited, steps, _ := BootLoop(input)
	fmt.Printf("=== Part A ===\nAcc: %d,\nVisited: %v\nSteps: %v\n", acc, visited, steps)

	accB, lineChanged, stepsB := BruteForceSequence(input)
	fmt.Printf("\n=== Part B ===\nAcc: %d,\nLine changed: %v\nSteps: %v\n", accB, lineChanged, stepsB)
}
