package main

import (
	"reflect"
	"testing"
)

func TestSeatInfo(t *testing.T) {
	seatInfoVerify(t, "FBFBBFFRLR", 44, 5, 357)
	seatInfoVerify(t, "BFFFBBFRRR", 70, 7, 567)
	seatInfoVerify(t, "FFFBBBFRRR", 14, 7, 119)
	seatInfoVerify(t, "BBFFBBFRLL", 102, 4, 820)
}

func seatInfoVerify(t *testing.T, seat string, row int, column int, id int) {
	result := SeatInfo(seat)
	if result.row != row || result.col != column || result.id != id {
		t.Errorf("%v\nExpected:\t%d,%d,%d\nGot\t\t%d,%d,%d", seat, row, column, id, result.row, result.col, result.id)
	}
}

func TestFindHighestId(t *testing.T) {
	expected := Seat{"BBFFBBFRLL", 102, 4, 820}
	seats := []string{
		"FBFBBFFRLR",
		"BBFFBBFRLL",
		"BFFFBBFRRR",
		"FFFBBBFRRR"}
	result := FindHighestId(seats)
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}
