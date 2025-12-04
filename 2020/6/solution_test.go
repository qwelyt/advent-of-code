package main

import "testing"

func TestCountGroupAnyYes(t *testing.T) {
	verifyAnyGroupCount(t, []string{"abc"}, 3)
	verifyAnyGroupCount(t, []string{"a", "b", "c"}, 3)
}

func TestCountGroupAllYes(t *testing.T) {
	verifyAllGroupCount(t, []string{"abc"}, 3)
	verifyAllGroupCount(t, []string{"a", "b", "c"}, 0)
}

func verifyAnyGroupCount(t *testing.T, input []string, expected int) {
	result := CountGroupAnyYes(input)
	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}

func verifyAllGroupCount(t *testing.T, input []string, expected int) {
	result := CountGroupAllYes(input)
	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}

func TestCountAnyYes(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 11
	result := Count(input, CountGroupAnyYes)
	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}
func TestCountAllYes(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 6
	result := Count(input, CountGroupAllYes)
	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}
