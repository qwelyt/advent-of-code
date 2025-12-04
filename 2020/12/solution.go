package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

type Instruction struct {
	op     string
	amount int
}

func ReadFile(filePath string) []Instruction {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values []Instruction
	for scanner.Scan() {
		s := scanner.Text()
		i, err := strconv.Atoi(s[1:])
		if err != nil {
			log.Fatal(err)
		}
		ins := Instruction{string(s[0]), i}
		values = append(values, ins)
	}
	return values
}

func add(op string, amount int, ns int, ew int) (int, int) {
	switch op {
	case "N":
		ns -= amount
	case "S":
		ns += amount
	case "W":
		ew -= amount
	case "E":
		ew += amount
	}
	return ns, ew
}

func Travel(Instructions []Instruction) (int, int) {
	var NS, EW int
	dirs := []string{"N", "E", "S", "W"}
	curDir := 1
	for _, v := range Instructions {
		NS, EW = add(v.op, v.amount, NS, EW)
		switch v.op {
		case "F":
			NS, EW = add(dirs[curDir], v.amount, NS, EW)
		case "R":
			a := v.amount / 90
			curDir = (curDir + a) % len(dirs)
		case "L":
			a := v.amount / 90
			c := curDir - a
			if c < 0 {
				c += len(dirs)
			}
			curDir = c % len(dirs)
		}
	}
	return NS, EW
}

func abs(a int) int {
	return int(math.Abs(float64(a)))
}

type Position struct {
	y, x int
}

func Rotate(wp Position, amount int) Position {
	switch amount {
	case 1:
		// N -> E == -y -> +x
		// W -> N == -x -> -y
		// E -> S == +x -> +y
		// S -> W == +y -> -x
		return Position{wp.x, wp.y * -1}
	case 2:
		return Position{wp.y * -1, wp.x * -1}
	case 3:
		// N -> W == -y -> -x
		// W -> S == -x -> +y
		// S -> E == +y -> +x
		// E -> N == +x -> -y
		return Position{wp.x * -1, wp.y}
	case 4:
		return wp
	}

	return Position{}
}

func Sail(pos Position, wp Position, amount int) Position {
	x := wp.x * amount
	y := wp.y * amount
	return Position{pos.y + y, pos.x + x}
}

func WPTravel(instructions []Instruction) Position {
	var pos = Position{}
	var wp = Position{-1, 10}

	for _, v := range instructions {
		switch v.op {
		case "N":
			wp.y -= v.amount
		case "E":
			wp.x += v.amount
		case "S":
			wp.y += v.amount
		case "W":
			wp.x -= v.amount
		case "F":
			pos = Sail(pos, wp, v.amount)
		case "L":
			r := 4 - (v.amount / 90)
			wp = Rotate(wp, r)
		case "R":
			wp = Rotate(wp, v.amount/90)
		}
	}

	return pos
}

func main() {
	input := ReadFile("input.txt")
	{
		ns, ew := Travel(input)
		n, e := abs(ns), abs(ew)
		fmt.Printf("=== Part A ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
	{
		pos := WPTravel(input)
		n, e := abs(pos.y), abs(pos.x)
		fmt.Printf("\n=== Part B ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
}
