package main

import (
	"reflect"
	"testing"
)

func TestPartA(t *testing.T) {
	input := ReadFile("example.txt")
	end := RunSequence(input, 6, StateCheck)
	result := CountActive(end, "#")
	expected := 112

	if expected != result {
		t.Errorf("Expected: %d, got %d", expected, result)
	}
}

func TestStateCheck(t *testing.T) {
	var state = [][][]string{
		{ // z = 0
			{".", "#", "."},
			//    y=1
			{"." /*x=0*/, ".", "#"},
			{"#", "#", "#"},
		},
	}
	change, value := StateCheck(0, 1, 0, state) // mid left
	if !change {
		t.Errorf("Should change")
	}
	if value == "." {
		t.Errorf("Gave wrong value")
	}
}

func TestStateCheck2(t *testing.T) {
	var state = [][][]string{
		{
			{".", "#", "."},
			{".", ".", "#"},
			{"#", "#", "#"},
		},
	}
	change, value := StateCheck(0, 2, 2, state) // Check bottom right
	if change {
		t.Errorf("Should not change")
	}
	if value == "." {
		t.Errorf("Gave wrong value")
	}
}

func TestStateCheck3(t *testing.T) {
	var state = [][][]string{
		{
			{".", "#", "."},
			{".", ".", "#"},
			{"#", "#", "#"},
		},
	}
	change, value := StateCheck(0, 2, 0, state) // Check bottom right
	if !change {
		t.Errorf("Should change")
	}
	if value != "." {
		t.Errorf("Gave wrong value")
	}
}

func TestCopy(t *testing.T) {
	var in = [][][]string{
		{
			{".", "#", "."},
			{".", ".", "#"},
			{"#", "#", "#"},
		},
	}
	out := Copy(in)

	if !reflect.DeepEqual(out, in) {
		t.Errorf("Did not copy correctly")
	}
}

func TestExpand(t *testing.T) {
	var in = [][][]string{
		{
			{".", "#", "."},
			{".", ".", "#"},
			{"#", "#", "#"},
		},
	}
	var expected = [][][]string{
		{
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
		},
		{
			{".", ".", ".", ".", "."},
			{".", ".", "#", ".", "."},
			{".", ".", ".", "#", "."},
			{".", "#", "#", "#", "."},
			{".", ".", ".", ".", "."},
		},
		{
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
			{".", ".", ".", ".", "."},
		},
	}
	result := Expand(in)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}

}
