package main

import (
	"reflect"
	"testing"
)

func TestFindPath(t *testing.T) {
	input := ReadFile("example.txt")
	expected := Path{10, 3, 7}
	result := FindPath(input, ".", "#", 3, 1)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect. Wanted %v, got %v", expected, result)
	}
}

func TestPartA(t *testing.T) {
	input := ReadFile("example.txt")
	expected := Path{10, 3, 7}
	result := PartA(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect. Wanted %v, got %v", expected, result)
	}
}
func TestPartB(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 336
	result := PartB(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect. Wanted %v, got %v", expected, result)
	}
}
