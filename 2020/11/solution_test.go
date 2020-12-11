package main

import (
	"reflect"
	"testing"
)

func TestStabilize(t *testing.T) {
	input := ReadFile("example.txt")
	expected := [][]string{
		{"#", ".", "#", "L", ".", "L", "#", ".", "#", "#"},
		{"#", "L", "L", "L", "#", "L", "L", ".", "L", "#"},
		{"L", ".", "#", ".", "L", ".", ".", "#", ".", "."},
		{"#", "L", "#", "#", ".", "#", "#", ".", "L", "#"},
		{"#", ".", "#", "L", ".", "L", "L", ".", "L", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
		{".", ".", "L", ".", "L", ".", ".", ".", ".", "."},
		{"#", "L", "#", "L", "#", "#", "L", "#", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "L", "L", ".", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
	}
	result := Stabilize(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\n%v\nGot:\n%v", format(expected), format(result))
	}
}

func TestCount(t *testing.T) {
	input := [][]string{
		{"#", ".", "#", "L", ".", "L", "#", ".", "#", "#"},
		{"#", "L", "L", "L", "#", "L", "L", ".", "L", "#"},
		{"L", ".", "#", ".", "L", ".", ".", "#", ".", "."},
		{"#", "L", "#", "#", ".", "#", "#", ".", "L", "#"},
		{"#", ".", "#", "L", ".", "L", "L", ".", "L", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
		{".", ".", "L", ".", "L", ".", ".", ".", ".", "."},
		{"#", "L", "#", "L", "#", "#", "L", "#", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "L", "L", ".", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
	}
	expected := 37
	result := CountSeats(input, "#")

	if expected != result {
		t.Errorf("Expected: %v, Got: %v", expected, result)
	}
}

func format(a [][]string) string {
	var s string
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[i]); j++ {
			s = s + a[i][j] + " "
		}
		s = s + "\n"
	}
	return s
}
