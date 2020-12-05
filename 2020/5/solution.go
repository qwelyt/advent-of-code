package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Seat struct {
	str          string
	row, col, id int
}

type EmptySeat struct {
	x, y, id int
}

func SeatId(row int, col int) int {
	return row*8 + col
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

func SeatInfo(seat string) Seat {
	var row, column int
	rMin, rMax := 0, 128
	cMin, cMax := 0, 8
	rowString := seat[0:7]
	columnString := seat[7:]

	for i := 0; i < len(rowString); i++ {
		c := string(rowString[i])
		h := (rMax - rMin) / 2
		switch c {
		case "B":
			rMin += h
		case "F":
			rMax -= h
		}
	}

	for i := 0; i < len(columnString); i++ {
		c := string(columnString[i])
		h := (cMax - cMin) / 2
		switch c {
		case "R":
			cMin += h
		case "L":
			cMax -= h
		}
	}

	if rMin != rMax-1 {
		fmt.Printf("Rows are wrong!, %d,%d\n", rMin, rMax)
	}
	if cMin != cMax-1 {
		fmt.Printf("Cols are wrong! %d,%d\n", cMin, cMax)
	}

	row = rMin
	column = cMin
	id := SeatId(row, column)
	return Seat{seat, row, column, id}
}

func FindHighestId(seats []string) Seat {
	var r, c, i, max int
	var seat string
	for _, v := range seats {
		s := SeatInfo(v)
		if s.id > max {
			r = s.row
			c = s.col
			i = s.id
			max = s.id
			seat = s.str
		}
	}
	return Seat{seat, r, c, i}
}

func FindEmptySeat(seats []string) []EmptySeat {
	var seatMap = make(map[int]Seat)
	var emptySeats []EmptySeat
	var maxRow, maxCol int

	for _, seatCode := range seats {
		seat := SeatInfo(seatCode)
		seatMap[seat.id] = seat
		if seat.row > maxRow {
			maxRow = seat.row
		}
		if seat.col > maxCol {
			maxCol = seat.col
		}
	}
	for r := 0; r < maxRow; r++ {
		for c := 0; c < maxCol; c++ {
			id := SeatId(r, c)
			_, taken := seatMap[id]
			if !taken {
				_, lower := seatMap[id-1]
				_, higher := seatMap[id+1]
				if lower && higher {
					emptySeats = append(emptySeats, EmptySeat{r, c, id})
				}
			}
		}
	}
	return emptySeats
}

func main() {
	input := ReadFile("input.txt")
	partA := FindHighestId(input)
	fmt.Printf("=== Part A ===\nSeat: %v\n", partA)
	partB := FindEmptySeat(input)
	fmt.Printf("=== Part B ===\nEmpty seats: %v\n", partB)
}
