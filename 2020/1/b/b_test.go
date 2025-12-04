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
	expected := Tuple3{366, 979, 675}
	if result != expected {
		t.Errorf("Value incorrect. Wanted %d, got %d", expected, result)
	}
}

func TestMultiply(t *testing.T) {
	tuple := Tuple3{979, 366, 675}
	result := Multiply(tuple)
	expected := 241861950
	if result != expected {
		t.Errorf("Value incorrect. Wanted %d, got %d", expected, result)
	}
}

// func TestDebug(t *testing.T) {
// 	Debug()
// }
