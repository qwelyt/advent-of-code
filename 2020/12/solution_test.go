package main

import (
	"reflect"
	"testing"
)

func TestTravel(t *testing.T) {
	input := ReadFile("example.txt")
	expectedNS, expectedEW := 8, 17
	rNS, rEW := Travel(input)

	if expectedEW != rEW || expectedNS != rNS {
		t.Errorf("Expected (%d,%d), got (%d,%d)", expectedNS, expectedEW, rNS, rEW)
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
		t.Errorf("Expected (%d,%d), got (%d,%d)", eNS, eEW, rNS, rEW)
	}
}

func TestWPTravel(t *testing.T) {
	input := ReadFile("example.txt")
	expected := Position{72, 214}
	result := WPTravel(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
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
	expected := Position{3, 71}
	result := WPTravel(input)
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}

func TestOwnWPTravel2(t *testing.T) {
	input := []Instruction{
		// wp1d == -1, wp2d == 10
		// wp1 == 0, wp2 == 1
		{"S", 2},  // wp1d == 1, wp2d == 10
		{"F", 10}, // NS == 10, EW == 100
	}
	expected := Position{10, 100}
	result := WPTravel(input)
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
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
	expected := Position{20, 0}
	result := WPTravel(input)
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
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
	expected := Position{107, 1293}
	result := WPTravel(input)
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}

func TestOwnWPTravel5(t *testing.T) {
	input := []Instruction{
		// pos{0,0} wp{-1,10}
		{"S", 1},   // pos{0,0} wp{0,10}
		{"F", 10},  // pos{0,100} wp{0,10}
		{"R", 90},  // pos{0,100} wp{10,-0}
		{"F", 2},   // pos{20,100} wp{10,-0}
		{"N", 1},   // pos{20,100} wp{9,-0}
		{"W", 2},   // pos{20,100} wp{9,-2}
		{"F", 5},   // pos{65,90} wp{9,-2}
		{"E", 10},  // pos{65,90} wp{9,8}
		{"F", 10},  // pos{155,170} wp{9,8}
		{"L", 90},  // pos{155,170} wp{-8,9}
		{"F", 3},   // pos{131,197} wp{-8,9}
		{"L", 180}, // pos{131,197} wp{8,-9}
		{"F", 6},   // pos{179,143} wp{8,-9}
		{"L", 270}, // pos{179,143} wp{-9, -8}
		{"F", 2},   // pos{161, 127} wp{-9,-8}
	}
	expected := Position{161, 127}
	result := WPTravel(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}
