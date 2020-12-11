package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

var freeSeat = "L"
var occupied = "#"
var floor = "."

func ReadFile(filePath string) [][]string {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values [][]string
	for scanner.Scan() {
		t := strings.Split(scanner.Text(), "")
		values = append(values, t)
	}

	return values
}

func EmptyArr(in [][]string) [][]string {
	var out [][]string
	// Initialize to empty
	for i := 0; i < len(in); i++ {
		var arr []string
		for j := 0; j < len(in[i]); j++ {
			arr = append(arr, "")
		}
		out = append(out, arr)
	}
	return out
}

func OccupyAllSeats(in [][]string) [][]string {
	out := EmptyArr(in)

	for i := 0; i < len(in); i++ {
		for j := 0; j < len(in[i]); j++ {
			if in[i][j] == freeSeat {
				out[i][j] = occupied
			} else {
				out[i][j] = in[i][j]
			}
		}
	}
	return out
}

func AdjacentCheck(seats [][]string, y int, x int, checkFor map[string]bool, goal int) bool {
	var num int

	for i := -1; i < len(seats) && i < 2; i++ {
		yi := y + i
		if yi < 0 || yi > len(seats)-1 {
			continue
		}
		for j := -1; j < len(seats[yi]) && j < 2; j++ {
			xj := x + j
			if xj < 0 || xj > len(seats[yi])-1 {
				continue
			}
			if yi == y && xj == x {
				continue
			}
			if checkFor[seats[yi][xj]] {
				num++
				if num >= goal {
					return true
				}
			}
		}
	}
	if num >= goal {
		return true
	}

	return false
}

func FreeSeatCheck(seats [][]string, y int, x int, checkFor map[string]bool) bool {
	for i := -1; i < len(seats) && i < 2; i++ {
		yi := y + i
		if yi < 0 || yi > len(seats)-1 {
			continue
		}
		for j := -1; j < len(seats[yi]) && j < 2; j++ {
			xj := x + j
			if xj < 0 || xj > len(seats[yi])-1 {
				continue
			}
			if yi == y && xj == x {
				continue
			}
			if checkFor[seats[yi][xj]] {
				return false
			}
		}
	}
	return true

}

func ShouldChange(seats [][]string, y int, x int) (bool, string) {
	seat := seats[y][x]
	occupiedCheck := map[string]bool{
		occupied: true,
	}

	switch seat {
	case freeSeat:
		if !AdjacentCheck(seats, y, x, occupiedCheck, 1) {
			// if FreeSeatCheck(seats, y, x, occupiedCheck) {
			return true, occupied
		}
	case occupied:
		if AdjacentCheck(seats, y, x, occupiedCheck, 4) {
			return true, freeSeat
		}
	case floor:
		return false, floor
	}
	return false, seat
}

func Equals(a [][]string, b [][]string) bool {
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[i]); j++ {
			if a[i][j] != b[i][j] {
				return false
			}
		}
	}
	return true

}

func Stabilize(in [][]string) [][]string {
	var previous = OccupyAllSeats(in)
	current := EmptyArr(in)

	for {
		for i := 0; i < len(previous); i++ {
			for j := 0; j < len(previous[i]); j++ {
				if y, v := ShouldChange(previous, i, j); y {
					current[i][j] = v
				} else {
					current[i][j] = previous[i][j]
				}
			}
		}
		if Equals(previous, current) {
			break
		}
		for i := 0; i < len(current); i++ {
			// fmt.Println(current[i])
			for j := 0; j < len(current[i]); j++ {
				previous[i][j] = current[i][j]
			}
		}
		// fmt.Println()
	}

	return previous
}

func CountSeats(in [][]string, seat string) int {
	var ans int
	for i := 0; i < len(in); i++ {
		for j := 0; j < len(in[i]); j++ {
			if in[i][j] == seat {
				ans++
			}
		}
	}
	return ans
}

func main() {
	input := ReadFile("input.txt")
	stable := Stabilize(input)
	// fmt.Println(stable)
	fmt.Printf("=== Part A ===\nOccupied seats: %d\n\n", CountSeats(stable, occupied))
}
