package main

import (
	"reflect"
	"testing"
)

func TestProcessInput(t *testing.T) {
	input := []string{"1-3 a: abcde"}
	expected := []Password{Password{1, 3, "a", "abcde"}}
	result := ProcessInput(input)
	if len(result) != len(expected) {
		t.Errorf("Value incorrect. Wanted %v, got %v", expected, result)
	}
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect. Wanted %v, got %v", expected, result)
	}
}

func TestProcessInput_example(t *testing.T) {

	expected := []Password{{1, 3, "a", "abcde"}, {1, 3, "b", "cdefg"}, {2, 9, "c", "ccccccccc"}}

	inputData := ReadFile("example.txt")
	result := ProcessInput(inputData)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect.\nWanted:\t%v\ngot:\t%v", expected, result)
	}
}

func TestValidPasswords_partA_valid(t *testing.T) {
	input := []Password{Password{1, 3, "a", "abcde"}}
	valid, invalid := ValidPasswords_partA(input)
	if valid == 0 || invalid != 0 {
		t.Errorf("Value incorrect. Expected 1 valid and 0 invalid. Got %d valid and %d invalid", valid, invalid)
	}
}

func TestValidPasswords_partA_invalid(t *testing.T) {
	input := []Password{{1, 3, "b", "cdefg"}}
	valid, invalid := ValidPasswords_partA(input)
	if valid != 0 || invalid == 0 {
		t.Errorf("Value incorrect. Expected 0 valid and 1 invalid. Got %d valid and %d invalid", valid, invalid)
	}
}

func TestValidPasswords_partB_valid(t *testing.T) {
	input := []Password{Password{1, 3, "a", "abcde"}}
	valid, invalid := ValidPasswords_partB(input)
	if valid == 0 || invalid != 0 {
		t.Errorf("Value incorrect. Expected 1 valid and 0 invalid. Got %d valid and %d invalid", valid, invalid)
	}
}
func TestValidPasswords_partB_invalid(t *testing.T) {
	input := []Password{{1, 3, "b", "cdefg"}}
	valid, invalid := ValidPasswords_partB(input)
	if valid != 0 || invalid == 0 {
		t.Errorf("Value incorrect. Expected 0 valid and 1 invalid. Got %d valid and %d invalid", valid, invalid)
	}
}

func TestValidPasswords_partB(t *testing.T) {
	input := []Password{{1, 3, "a", "abcde"}, {1, 3, "b", "cdefg"}, {2, 9, "c", "ccccccccc"}}
	valid, invalid := ValidPasswords_partB(input)
	if valid != 1 {
		t.Errorf("Expected 1 valid. Got %d", valid)
	}
	if invalid != 2 {
		t.Errorf("Expected 2 invalid. Got %d", invalid)
	}
}
