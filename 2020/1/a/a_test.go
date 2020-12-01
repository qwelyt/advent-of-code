package main

import (
	"reflect"
	"testing"
)

func TestReadFile(t *testing.T) {
	expected := []int{1721, 979, 366, 299, 675, 1456}
	result := ReadFile("../example.txt")
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Value incorrect. Wanted %d, got %d", expected, result)
	}
}

func TestSumToTarget(t *testing.T) {
	values := []int{1721, 979, 366, 299, 675, 1456}
	result := SumToTarget(values, 2020)
	expected := Pair{1721, 299}
	if result != expected {
		t.Errorf("Value incorrect. Wanted %d, got %d", expected, result)
	}
}

func TestMultiply(t *testing.T) {
	result := Multiply(1721, 299)
	expected := 514579
	if result != expected {
		t.Errorf("Value incorrect. Wanted %d, got %d", expected, result)
	}
}
