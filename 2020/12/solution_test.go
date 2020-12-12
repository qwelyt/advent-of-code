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

func TestWPTravel(t *testing.T) {
	input := ReadFile("example.txt")
	expectedNS, expectedEW := 72, 214
	rNS, rEW := WaypointTravel(input)

	if expectedEW != rEW || expectedNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", expectedNS, expectedEW, rNS, rEW)
	}
}

func TestOwnWPTravel(t *testing.T) {
	input := []Instruction{
		// wp1d == -1, wp2d == 10
		// wp1 == 0, wp2 == 1
		{"F", 10},  // EW == 100,NS == -10
		{"R", 90},  // 90 % 90 == 1 -> wp1,wp2 == 1,2 == E,S
		{"F", 1},   // EW == 101, NS == 0
		{"L", 270}, // 270%90 == 3 -> wp1,wp2 == 2,3 == S,W
		{"F", 3},   // EW == 71, NS == 3
	}
	eNS, eEW := 3, 71
	rNS, rEW := WaypointTravel(input)
	if eEW != rEW || eNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}

func TestOwnWPTravel2(t *testing.T) {
	input := []Instruction{
		// wp1d == -1, wp2d == 10
		// wp1 == 0, wp2 == 1
		{"S", 2},  // wp1d == 1, wp2d == 10
		{"F", 10}, // NS == 10, EW == 100
	}
	eNS, eEW := 10, 100
	rNS, rEW := WaypointTravel(input)
	if eEW != rEW || eNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}

func TestOwnWPTravel3(t *testing.T) {
	input := []Instruction{
		// wp1d == -1, wp2d == 10
		// wp1 == 0, wp2 == 1
		{"S", 2},  // wp1d == 1, wp2d == 10
		{"F", 10}, // NS == 10, EW == 100
		{"W", 20}, // wp1d == 1, wpd2d == -10
		{"F", 10}, // NS == 20, EW == 0
	}
	eNS, eEW := 20, 0
	rNS, rEW := WaypointTravel(input)
	if eEW != rEW || eNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}

func TestOwnWPTravel4(t *testing.T) {
	input := []Instruction{
		// wp1d == -1, wp2d == 10
		// wp1 == 0, wp2 == 1
		{"S", 1},   // wp1d == 0, wp2d == 10
		{"F", 17},  // NS == 0, EW == 170
		{"S", 3},   // wp1d == 3, wp2d == 10
		{"F", 56},  // NS == 168, EW == 730
		{"W", 5},   // wp1d == 3, wp2d == 5
		{"F", 11},  // NS == 201, EW == 785
		{"N", 4},   // wp1d == -1, wp2d == 5
		{"F", 94},  // NS == 107, EW == 1255
		{"W", 4},   // wp1d == -1, wp2d == 1
		{"S", 1},   // wp1d == 0, wp2d == 1
		{"L", 180}, // wp1 == 2, wp2 == 3 --> S,W
		// 				--> wp1d == 0, wp2d == -1
		{"E", 2},  // wp1d == 0, wp2d == 1
		{"F", 38}, // NS == 107, EW == 1293
	}
	eNS, eEW := 107, 1293
	rNS, rEW := WaypointTravel(input)
	if eEW != rEW || eNS != rNS {
		t.Errorf("Expecter (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}
