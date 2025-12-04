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

func TestBMasks(t *testing.T) {
	value := "6"
	for i := 0; i < 36; i++ {
		mask := make([]rune, 36)
		for i := range mask {
			mask[i] = '0'
		}
		mask[i] = 'X'
		result := ApplyMemMask(value, mask)
		if len(result) != 2 {
			t.Errorf("Should have produced two mask for mask: %v\n but got %v", mask, result)

		}
	}
}

func TestRunA(t *testing.T) {
	input := ReadFile("input.txt")
	ans := PartA(input)
	if ans != 10452688630537 {
		t.Errorf("Part A broke. Got %d", ans)
	}

}

func TestRunB(t *testing.T) {
	input := ReadFile("input.txt")
	ans := PartB(input)
	if ans != 2881082759597 {
		t.Errorf("Part B broke. Got %d", ans)
	}
}
