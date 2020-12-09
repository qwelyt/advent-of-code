package main

import "testing"

func TestFindInvalidXMAS(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 127
	result, position, _ := FindInvalidXMAS(input, 5)

	if expected != result {
		t.Errorf("Expected: %d, got: %d", expected, result)
	}
	if position != 14 {
		t.Errorf("Expected: %d, got: %d", 14, result)
	}
}

func TestFull(t *testing.T) {
	input := ReadFile("example.txt")
	partAValue, partAIndex, _ := FindInvalidXMAS(input, 5)
	numberRange := SumToNumber(input[:partAIndex], partAValue)
	var lowest, highest = LowHigh(numberRange)

	if lowest != 15 {
		t.Errorf("Expected: %d, got: %d", 15, lowest)
	}
	if highest != 47 {
		t.Errorf("Expected: %d, got: %d", 47, highest)
	}
}
