package main

import "testing"

func TestSolveA(t *testing.T) {
	var cases = map[int]string{
		26:    "2 * 3 + (4 * 5)",
		437:   "5 + (8 * 3 + 9 + 3 * 4 * 3)",
		12240: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
		13632: "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
	}

	for expected, input := range cases {
		result := SolveEquation(input, LTRSolve)
		if expected != result {
			t.Errorf("Expected: %d, got %d", expected, result)
		}
	}
}

func TestLTRSolve(t *testing.T) {
	var cases = map[int]string{
		3:  "1 + 2",
		2:  "1 + 1 * 1",
		24: "3 + 3 * 4",
		0:  "3 + 2 - 1 * 0",
		10: "10 - 1 * 2 + 2 / 2",
	}
	for expected, input := range cases {
		result := LTRSolve(input)
		if expected != result {
			t.Errorf("Equation: %v\nExpected: %d, got %d", input, expected, result)
		}
	}
}

func TestAOMSolve(t *testing.T) {
	var cases = map[int]string{
		3:  "1 + 2",
		2:  "1 + 1 * 1",
		24: "3 + 3 * 4",
		0:  "3 + 2 - 1 * 0",
		10: "2 * 2 + 3",
		18: "10 - 1 * 2 + 2 / 2", // 10 - 1 * 4 / 2 -> LTRSolve -> 9 * 4 / 2 -> 36 / 2 -> 18
	}
	for expected, input := range cases {
		result := AOMSolve(input)
		if expected != result {
			t.Errorf("Equation: %v\nExpected: %d, got %d", input, expected, result)
		}
	}
}

func TestSolveEquationLTR(t *testing.T) {
	var cases = map[int]string{
		82: "1 + 2 + (3 * 4) + ((5 + 6) + (7 * 8))",
	}
	for expected, input := range cases {
		result := SolveEquation(input, LTRSolve)
		if expected != result {
			t.Errorf("Equation: %v\nExpected: %d, got %d", input, expected, result)
		}
	}
}

func TestSolveEquationAOM(t *testing.T) {
	var cases = map[int]string{
		51:     "1 + (2 * 3) + (4 * (5 + 6))",
		46:     "2 * 3 + (4 * 5)",
		1445:   "5 + (8 * 3 + 9 + 3 * 4 * 3)",
		669060: "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
		23340:  "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
	}
	for expected, input := range cases {
		result := SolveEquation(input, AOMSolve)
		if expected != result {
			t.Errorf("Equation: %v\nExpected: %d, got %d", input, expected, result)
		}
	}
}
