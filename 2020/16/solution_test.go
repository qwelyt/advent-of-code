package main

import (
	"reflect"
	"testing"
)

func TestCorrelateFields(t *testing.T) {
	expected := map[int]string{
		0: "row",
		1: "class",
		2: "seat",
	}
	input := ReadFile("example2.txt")
	clean := RemoveInvalidTickets(input)
	result := CorrelateFields(clean)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v\n", expected, result)
	}
}

func TestPartB(t *testing.T) {
	input := ReadFile("input.txt")
	clean := RemoveInvalidTickets(input)
	columns := CorrelateFields(clean)
	sum := MultiplyDeparture(clean, columns)
	t.Errorf("\nExpected:\t%v\nGot:\t\t%v\n", 0, sum)
}
