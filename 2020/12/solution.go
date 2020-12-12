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
type Position struct {
	y, x int
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

func Travel(instructions []Instruction) Position {
	var pos = Position{}
	var wp = Position{0, 1}
	for _, v := range instructions {
		switch v.op {
		case "N":
			pos.y -= v.amount
		case "E":
			pos.x += v.amount
		case "S":
			pos.y += v.amount
		case "W":
			pos.x -= v.amount
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

func abs(a int) int {
	return int(math.Abs(float64(a)))
}
func main() {
	input := ReadFile("input.txt")
	{
		pos := Travel(input)
		n, e := abs(pos.y), abs(pos.x)
		fmt.Printf("=== Part A ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
	{
		pos := WPTravel(input)
		n, e := abs(pos.y), abs(pos.x)
		fmt.Printf("\n=== Part B ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
}
