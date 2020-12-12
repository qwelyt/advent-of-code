package main

import "testing"

func TestTravel(t *testing.T) {
	input := ReadFile("example.txt")
	expectedNS, expectedEW := 8, 17
	rNS, rEW := Travel(input)

	if expectedEW != rEW || expectedNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", expectedNS, expectedEW, rNS, rEW)
	}
}

func TestOwnTravel(t *testing.T) {
	input := []Instruction{
		{"F", 10},  // EW == 10
		{"R", 90},  // Dir == S
		{"F", 1},   // NS == 1
		{"L", 270}, // Dir == W
		{"F", 3},   // EW == 7
	}
	eNS, eEW := 1, 7
	rNS, rEW := Travel(input)
	if eEW != rEW || eNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}
