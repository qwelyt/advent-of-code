package main

import (
	"testing"
)

func TestJoltageJump(t *testing.T) {
	input := ReadFile("example.txt")
	one, three := JoltageJumps(input)

	if 7 != one {
		t.Errorf("Expected %d one jumps, got %d", 7, one)
	}
	if 5 != three {
		t.Errorf("Expected %d three jumps, got %d", 5, three)
	}
}
func TestJoltageJump3(t *testing.T) {
	input := ReadFile("sortedInput.txt")
	one, three := JoltageJumps(input)

	if 66 != one {
		t.Errorf("Expected %d one jumps, got %d", 7, one)
	}
	if 30 != three {
		t.Errorf("Expected %d three jumps, got %d", 5, three)
	}
}

func TestJoltageJump2(t *testing.T) {
	input := ReadFile("example2.txt")
	one, three := JoltageJumps(input)

	if 22 != one {
		t.Errorf("Expected %d one jumps, got %d", 22, one)
	}
	if 10 != three {
		t.Errorf("Expected %d three jumps, got %d", 10, three)
	}
}

func TestDistinctArrangements(t *testing.T) {
	input := ReadFile("example2.txt")
	expected := 19208
	result := DA(input)

	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}

func TestDP(t *testing.T) {
	input := ReadFile("example2.txt")
	expected := 19208
	fixed := []int{0}
	fixed = append(fixed, input...)
	fixed = append(fixed, fixed[len(fixed)-1]+3)

	result, _ := DP(0, fixed, make(map[int]int))

	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}

}
