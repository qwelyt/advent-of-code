package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func ReadFile(filePath string) [][][]string { // [z][y][x]
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values [][][]string
	var slice [][]string
	for scanner.Scan() {
		state := strings.Split(scanner.Text(), "")
		slice = append(slice, state)
	}
	values = append(values, slice)
	return values
}

func DecideState(myState string, active int) (bool, string) {
	switch myState {
	case "#": // Active
		if active == 2 || active == 3 {
			return false, myState
		} else {
			return true, "."
		}
	case ".": // Inactive
		if active == 3 {
			return true, "#"
		} else {
			return false, myState
		}

	}
	return false, myState
}

func StateCheck(z int, y int, x int, state [][][]string) (bool, string) {
	var active int
	for zd := z - 1; zd < z+2; zd++ {
		if zd < 0 || zd >= len(state) {
			continue
		}
		for yd := y - 1; yd < y+2; yd++ {
			if yd < 0 || yd >= len(state[zd]) {
				continue
			}
			for xd := x - 1; xd < x+2; xd++ {
				if xd < 0 || xd >= len(state[zd][yd]) {
					continue
				}
				if zd == z && yd == y && xd == x { // Hey that's me!
					continue
				}
				if state[zd][yd][xd] == "#" {
					active++
				}
			}
		}
	}
	myState := state[z][y][x]
	return DecideState(myState, active)
}
func StateCheck4d(z int, y int, x int, w int, state [][][][]string) (bool, string) {
	var active int
	for zd := z - 1; zd < z+2; zd++ {
		if zd < 0 || zd >= len(state) {
			continue
		}
		for yd := y - 1; yd < y+2; yd++ {
			if yd < 0 || yd >= len(state[zd]) {
				continue
			}
			for xd := x - 1; xd < x+2; xd++ {
				if xd < 0 || xd >= len(state[zd][yd]) {
					continue
				}
				for wd := w - 1; wd < w+2; wd++ {
					if wd < 0 || wd >= len(state[zd][yd][xd]) {
						continue
					}
					if zd == z && yd == y && xd == x && wd == w { // Hey that's me!
						continue
					}
					if state[zd][yd][xd][wd] == "#" {
						active++
					}
				}
			}
		}
	}
	myState := state[z][y][x][w]
	return DecideState(myState, active)
}

func Copy(a [][][]string) [][][]string {
	var out = make([][][]string, len(a))
	for z := 0; z < len(a); z++ {
		out[z] = make([][]string, len(a[z]))
		for y := 0; y < len(a[z]); y++ {
			out[z][y] = make([]string, len(a[z][y]))
			for x := 0; x < len(a[z][y]); x++ {
				out[z][y][x] = a[z][y][x]
			}
		}
	}
	return out
}
func Copy4d(a [][][][]string) [][][][]string {
	var out = make([][][][]string, len(a))
	for z := 0; z < len(a); z++ {
		out[z] = make([][][]string, len(a[z]))
		for y := 0; y < len(a[z]); y++ {
			out[z][y] = make([][]string, len(a[z][y]))
			for x := 0; x < len(a[z][y]); x++ {
				out[z][y][x] = make([]string, len(a[z][y][x]))
				for w := 0; w < len(a[z][y][x]); w++ {
					out[z][y][x][w] = a[z][y][x][w]
				}
			}
		}
	}
	return out
}

func Expand(a [][][]string) [][][]string {
	var out = make([][][]string, len(a)+2)
	for z := 0; z < len(out); z++ {
		out[z] = make([][]string, len(a[0])+2)
		for y := 0; y < len(out[z]); y++ {
			out[z][y] = make([]string, len(a[0][0])+2)
			for x := 0; x < len(out[z][y]); x++ {
				out[z][y][x] = "."
			}
		}
	}
	for z := 0; z < len(a); z++ {
		for y := 0; y < len(a[z]); y++ {
			for x := 0; x < len(a[z][y]); x++ {
				out[z+1][y+1][x+1] = a[z][y][x]
			}
		}
	}
	return out
}
func Expand4d(a [][][][]string) [][][][]string {
	var out = make([][][][]string, len(a)+2)
	for z := 0; z < len(out); z++ {
		out[z] = make([][][]string, len(a[0])+2)
		for y := 0; y < len(out[z]); y++ {
			out[z][y] = make([][]string, len(a[0][0])+2)
			for x := 0; x < len(out[z][y]); x++ {
				out[z][y][x] = make([]string, len(a[0][0][0])+2)
				for w := 0; w < len(out[z][y][x]); w++ {
					out[z][y][x][w] = "."
				}
			}
		}
	}
	for z := 0; z < len(a); z++ {
		for y := 0; y < len(a[z]); y++ {
			for x := 0; x < len(a[z][y]); x++ {
				for w := 0; w < len(a[z][y][x]); w++ {
					out[z+1][y+1][x+1][w+1] = a[z][y][x][w]
				}
			}
		}
	}
	return out
}

func RunSequence(input [][][]string, times int, changeFn func(int, int, int, [][][]string) (bool, string)) [][][]string {
	// RULES
	// If cube == active
	// 		If exactly 2 or exactly 3 neighbors are active, the cube remains active.
	//		Else it becomes inactive
	//
	// If cube == inactive
	//		If exactly 3 neighbors are active it becomes active
	//		Else it remains inactive
	var previous [][][]string
	var current [][][]string

	previous = Copy(Expand(input))
	current = Copy(Expand(input))

	for i := 0; i < times; i++ { // Primary loop
		for z := 0; z < len(previous); z++ {
			for y := 0; y < len(previous[z]); y++ {
				for x := 0; x < len(previous[z][y]); x++ {
					if yesno, value := changeFn(z, y, x, previous); yesno {
						current[z][y][x] = value
					} else {
						current[z][y][x] = previous[z][y][x]
					}
				}
			}
		}

		// Copy the state and expand the playing field
		current = Expand(current)
		previous = Copy(current)
	}
	// fmt.Println(previous)
	return current
}

func CountActive(board [][][]string, find string) int {
	var sum int
	for z := 0; z < len(board); z++ {
		for y := 0; y < len(board[z]); y++ {
			for x := 0; x < len(board[z][y]); x++ {
				if board[z][y][x] == find {
					sum += 1
				}
			}
		}
	}
	return sum
}
func CountActive4d(board [][][][]string, find string) int {
	var sum int
	for z := 0; z < len(board); z++ {
		for y := 0; y < len(board[z]); y++ {
			for x := 0; x < len(board[z][y]); x++ {
				for w := 0; w < len(board[z][y][x]); w++ {
					if board[z][y][x][w] == find {
						sum += 1
					}
				}
			}
		}
	}
	return sum
}

func RunSequence4d(input [][][][]string, times int, changeFn func(int, int, int, int, [][][][]string) (bool, string)) [][][][]string {
	var previous [][][][]string
	var current [][][][]string

	previous = Copy4d(Expand4d(input))
	current = Copy4d(Expand4d(input))

	for i := 0; i < times; i++ { // Primary loop
		for z := 0; z < len(previous); z++ {
			for y := 0; y < len(previous[z]); y++ {
				for x := 0; x < len(previous[z][y]); x++ {
					for w := 0; w < len(previous[z][y][x]); w++ {
						if yesno, value := changeFn(z, y, x, w, previous); yesno {
							current[z][y][x][w] = value
						} else {
							current[z][y][x][w] = previous[z][y][x][w]
						}
					}
				}
			}
		}

		// Copy the state and expand the playing field
		current = Expand4d(current)
		previous = Copy4d(current)
	}
	return current
}

func AddDimension(a [][][]string) [][][][]string {
	var out [][][][]string
	out = append(out, [][][]string{})
	out[0] = a
	return out
}

func main() {
	input := ReadFile("input.txt")
	{
		end := RunSequence(input, 6, StateCheck)
		result := CountActive(end, "#")
		fmt.Printf("=== Part A ===\n3d space active: %d\n\n", result)
	}
	{
		input4d := AddDimension(input)
		end := RunSequence4d(input4d, 6, StateCheck4d)
		result := CountActive4d(end, "#")
		fmt.Printf("=== Part B ===\n4d space active: %d\n\n", result)
	}
}
