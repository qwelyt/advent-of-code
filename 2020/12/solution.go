package main

import (
	"bufio"
	"fmt"
	"log"
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

func WaypointTravel(instructions []Instruction) (int, int) {
	var NS, EW int
	dirs := []string{"N", "E", "S", "W"}
	wpNS := 0
	wpEW := 1
	wpNSd := 1
	wpEWd := 10
	for _, v := range instructions {
		switch v.op {
		case "F":

		}
	}
}

func main() {
	input := ReadFile("input.txt")
	ns, ew := Travel(input)
	fmt.Printf("=== Part A ===\n(%d,%d) :: %d\n", ns, ew, ns+ew)
}
