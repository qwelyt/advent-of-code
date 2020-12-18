package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
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

func LTRSolve(eq string) int {
	var sum int
	var nextFunc = func(a int, b int) int { return b }
	e := strings.Split(eq, " ")
	for _, o := range e {
		switch string(o) {
		case " ":
			continue
		case "+":
			nextFunc = func(a int, b int) int { return a + b }
		case "-":
			nextFunc = func(a int, b int) int { return a - b }
		case "*":
			nextFunc = func(a int, b int) int { return a * b }
		case "/":
			nextFunc = func(a int, b int) int { return a / b }
		default:
			i, err := strconv.Atoi(o)
			if err != nil {
				log.Fatal(err)
			}
			sum = nextFunc(sum, i)
		}
	}
	return sum
}

func AOMSolve(eq string) int {
	return 0
}

func SolveEquation(eq string, solver func(string) int) int {
	var f, b = -1, len(eq)

	// 1 + 2 + ( 3 * 4 ) + (( 5 + 6 ) + ( 7 * 8))
	// 1 + 2 + 12    + ( 11       +   56    )
	// 1 + 2 + 12    +  67
	e := eq
	checkEnd := false
	var sum = 0
	for {
		for i := 0; i < len(e); i++ {
			checkEnd = true
			if e[i] == '(' {
				f = i
			}
			if e[i] == ')' {
				b = i
				// Now solve the sub equation and redo the loop
				sub := e[f+1 : b]
				subSolved := solver(sub)
				e = e[0:f] + strconv.Itoa(subSolved) + e[b+1:]
				f, b = -1, len(eq)
				checkEnd = false
				break
			}
		}
		if checkEnd && f == -1 && b == len(eq) {
			sum = LTRSolve(e)
			break
		}
	}
	return sum
}

func SolveA(equations []string) int64 {
	// 26:    "2 * 3 + (4 * 5)", <-- Split on ( and calculate left to right in each sub array
	// 437:   "5 + (8 * 3 + 9 + 3 * 4 * 3)", <-- Same ase above
	// 12240: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", <-- Split, same as above
	// 13632: "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", <-- now we need to extract on ( and ) to get sub arrays to calculate left to right
	var answers []int
	for _, eq := range equations {
		answers = append(answers, SolveEquation(eq, LTRSolve))
	}
	var sum int64
	for _, i := range answers {
		sum += int64(i)
	}
	return sum
}

func main() {
	input := ReadFile("input.txt")
	{
		partA := SolveA(input)
		fmt.Printf("=== Part A ===\nSum of all: %d\n", partA)
	}
}
