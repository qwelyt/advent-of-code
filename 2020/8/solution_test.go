package main

import (
	"reflect"
	"testing"
)

func TestBootLoop(t *testing.T) {
	input := ReadFile("example.txt")
	acc, visited, steps, finishedAll := BootLoop(input)
	expectedSteps := []int{0, 1, 2, 6, 7, 3, 4, 1}

	if acc != 5 {
		t.Errorf("Expected 5, got %d", acc)
	}
	if len(visited) != 7 {
		t.Errorf("Expected 7, got %d", len(visited))
	}
	if finishedAll {
		t.Errorf("Should not have finish, should have found inifint loop")
	}
	if !reflect.DeepEqual(expectedSteps, steps) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v", expectedSteps, steps)
	}
}

func TestSolveBootSequence(t *testing.T) {
	input := ReadFile("example.txt")
	// acc, lineChanged := SolveBootSequence(input)
	expectedSteps := []int{0, 1, 2, 6, 7, 8}
	acc, lineChanged, steps := BruteForceSequence(input)

	if acc != 8 {
		t.Errorf("Expected 8, got %d", acc)
	}
	if lineChanged != 7 {
		t.Errorf("Expected 7, got %d", lineChanged)
	}
	if !reflect.DeepEqual(expectedSteps, steps) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v", expectedSteps, steps)
	}
}
