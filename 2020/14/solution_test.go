package main

import (
	"testing"
)

func TestPartA(t *testing.T) {
	input := ReadFile("example.txt")
	var expected uint64
	expected = 165
	result := PartA(input)

	if expected != result {
		t.Errorf("Expected %v, Got %v", expected, result)
	}
}
func TestPartB(t *testing.T) {
	input := ReadFile("example2.txt")
	var expected uint64
	expected = 208
	result := PartB(input)

	if expected != result {
		t.Errorf("Expected %v, Got %v", expected, result)
	}
}

func TestOneMask(t *testing.T) {
	input := []string{
		"mask = 000000000000000000000000000000X1001X",
		"mem[42] = 100",
	}
	var expected uint64
	expected = 400
	result := PartB(input)

	if expected != result {
		t.Errorf("Expected %v, Got %v", expected, result)
	}
}

func TestBMoreMasks(t *testing.T) {
	input := []string{
		"mask = 000000000000000000000000000000X1001X",
		"mem[42] = 100",
		"mask = 00000000000000000000000000000000X0XX",
		"mem[26] = 1",
		"mem[27] = 1",
	}
	var expected uint64
	expected = 208
	result := PartB(input)

	if expected != result {
		t.Errorf("Expected %v, Got %v", expected, result)
	}
}

func TestRunB(t *testing.T) {
	input := ReadFile("input.txt")
	ans := PartB(input)
	// 703552866308
	// 697647875308
	if ans <= 703552866308 {
		t.Errorf("Answer is too low: %d", ans)
	}
	t.Errorf("Ans: %d", ans)
}
