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

func WaypointTravel(instructions []Instruction) (int, int) {
	var NS, EW int
	dirs := []string{"N", "E", "S", "W"}
	wp1 := 0
	wp2 := 1
	wp1d := -1
	wp2d := 10
	for _, v := range instructions {
		wp1dOld, wp2dOld := wp1d, wp2d
		wp1d, wp2d = add(v.op, v.amount, wp1d, wp2d)
		if wp1dOld != 0 && wp1dOld != wp1d && ((wp1dOld <= 0 && wp1d >= 0) || (wp1dOld >= 0 && wp1d <= 0)) {
			wp1 = (wp1 + 2) % len(dirs)
		}
		if wp2dOld != 0 && wp2dOld != wp2d && ((wp2dOld <= 0 && wp2d >= 0) || (wp2dOld >= 0 && wp2d <= 0)) {
			wp2 = (wp2 + 2) % len(dirs)
		}
		switch v.op {
		case "F":
			w1 := v.amount * abs(wp1d)
			w2 := v.amount * abs(wp2d)
			NS, EW = add(dirs[wp1], w1, NS, EW)
			NS, EW = add(dirs[wp2], w2, NS, EW)
		case "R":
			wp1Old, wp2Old := wp1, wp2
			a := v.amount / 90
			wp1 = (wp1 + a) % len(dirs)
			wp2 = (wp2 + a) % len(dirs)
			if a == 2 || (a == 1 && wp1 != 2) { // a = =3?

			}
			if a == 2 || (wp1Old == 0 && wp1 == 1) || (wp1Old == 2 && wp1 == 3) {
				wp1d *= -1
			}
			if a == 2 || (wp2Old == 0 && wp2 == 1) || (wp2Old == 2 && wp2 == 3) {
				wp2d *= -1
			}
		case "L":
			wp1Old, wp2Old := wp1, wp2
			a := v.amount / 90
			c1 := wp1 - a
			if c1 < 0 {
				c1 += len(dirs)
			}
			wp1 = c1 % len(dirs)
			c2 := wp2 - a
			if c2 < 0 {
				c2 += len(dirs)
			}
			wp2 = c2 % len(dirs)
			if a == 2 || (wp1Old == 1 && wp1 == 0) || (wp1Old == 3 && wp1 == 2) {
				wp1d *= -1
			}
			if a == 2 || (wp2Old == 1 && wp2 == 0) || (wp2Old == 3 && wp2 == 2) {
				wp2d *= -1
			}
		}
	}
	return NS, EW
}

func main() {
	input := ReadFile("input.txt")
	{
		ns, ew := Travel(input)
		n, e := abs(ns), abs(ew)
		fmt.Printf("=== Part A ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
	{
		ns, ew := WaypointTravel(input)
		n, e := abs(ns), abs(ew)
		fmt.Printf("\n=== Part B ===\n(%d,%d) :: %d\n", n, e, n+e)
	}
}
