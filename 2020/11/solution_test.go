package main

import (
	"reflect"
	"testing"
)

func TestStabilize(t *testing.T) {
	input := ReadFile("example.txt")
	expected := [][]string{
		{"#", ".", "#", "L", ".", "L", "#", ".", "#", "#"},
		{"#", "L", "L", "L", "#", "L", "L", ".", "L", "#"},
		{"L", ".", "#", ".", "L", ".", ".", "#", ".", "."},
		{"#", "L", "#", "#", ".", "#", "#", ".", "L", "#"},
		{"#", ".", "#", "L", ".", "L", "L", ".", "L", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
		{".", ".", "L", ".", "L", ".", ".", ".", ".", "."},
		{"#", "L", "#", "L", "#", "#", "L", "#", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "L", "L", ".", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
	}
	result := Stabilize(input, ShouldChange)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\n%v\nGot:\n%v", format(expected), format(result))
	}
}
func TestStabilizeBySight(t *testing.T) {
	input := ReadFile("example.txt")
	expected := [][]string{
		{"#", ".", "L", "#", ".", "L", "#", ".", "L", "#"},
		{"#", "L", "L", "L", "L", "L", "L", ".", "L", "L"},
		{"L", ".", "L", ".", "L", ".", ".", "#", ".", "."},
		{"#", "#", "L", "#", ".", "#", "L", ".", "L", "#"},
		{"L", ".", "L", "#", ".", "L", "L", ".", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "#", ".", "L", "L"},
		{".", ".", "#", ".", "L", ".", ".", ".", ".", "."},
		{"L", "L", "L", "#", "#", "#", "L", "L", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "L", "#", ".", "L"},
		{"#", ".", "L", "#", "L", "L", "#", ".", "L", "#"},
	}
	result := Stabilize(input, ShouldChangeBySight)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\n%v\nGot:\n%v", format(expected), format(result))
	}
}

func TestCount(t *testing.T) {
	input := [][]string{
		{"#", ".", "#", "L", ".", "L", "#", ".", "#", "#"},
		{"#", "L", "L", "L", "#", "L", "L", ".", "L", "#"},
		{"L", ".", "#", ".", "L", ".", ".", "#", ".", "."},
		{"#", "L", "#", "#", ".", "#", "#", ".", "L", "#"},
		{"#", ".", "#", "L", ".", "L", "L", ".", "L", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
		{".", ".", "L", ".", "L", ".", ".", ".", ".", "."},
		{"#", "L", "#", "L", "#", "#", "L", "#", "L", "#"},
		{"#", ".", "L", "L", "L", "L", "L", "L", ".", "L"},
		{"#", ".", "#", "L", "#", "L", "#", ".", "#", "#"},
	}
	expected := 37
	result := CountSeats(input, "#")

	if expected != result {
		t.Errorf("Expected: %v, Got: %v", expected, result)
	}
}

func TestSightCheck(t *testing.T) {
	checkFor := map[string]bool{"#": true}
	{
		input := [][]string{
			{".", ".", "."},
			{".", "L", "."},
			{".", ".", "."},
		}
		result := SightCheck(input, 1, 1, checkFor)
		if result != 0 {
			t.Errorf("Should have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", "#", "."},
			{".", "L", "."},
			{".", ".", "."},
		}
		result := SightCheck(input, 1, 1, checkFor)
		if result != 1 {
			t.Errorf("(0,1)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "."},
			{"#", "L", "."},
			{".", ".", "."},
		}
		result := SightCheck(input, 1, 1, checkFor)
		if result != 1 {
			t.Errorf("(1,0)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "."},
			{".", "L", "."},
			{".", "#", "."},
		}
		result := SightCheck(input, 1, 1, checkFor)
		if result != 1 {
			t.Errorf("(2,1)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "."},
			{".", "L", "#"},
			{".", ".", "."},
		}
		result := SightCheck(input, 1, 1, checkFor)
		if result != 1 {
			t.Errorf("(1,2)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{"#", ".", "."},
			{".", "L", "."},
			{".", ".", "."},
		}
		if SightCheck(input, 1, 1, checkFor) != 1 {
			t.Errorf("(0,0)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "#"},
			{".", "L", "."},
			{".", ".", "."},
		}
		if SightCheck(input, 1, 1, checkFor) != 1 {
			t.Errorf("(0,2)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "."},
			{".", "L", "."},
			{"#", ".", "."},
		}
		if SightCheck(input, 1, 1, checkFor) != 1 {
			t.Errorf("(2,0)Should not have found valid empty seat")
		}
	}
	{
		input := [][]string{
			{".", ".", "."},
			{".", "L", "."},
			{".", ".", "#"},
		}
		if SightCheck(input, 1, 1, checkFor) != 1 {
			t.Errorf("(2,2)Should not have found valid empty seat")
		}
	}
}

func TestSightCheck1(t *testing.T) {
	input := ReadFile("example2.txt")
	expected := 8
	result := SightCheck(input, 4, 3, map[string]bool{"#": true})

	if expected != result {
		t.Errorf("Should have found %d invalid, found %d", expected, result)
	}

}

func TestSightCheck2(t *testing.T) {
	input := ReadFile("example3.txt")
	expected := 0
	result := SightCheck(input, 1, 1, map[string]bool{"#": true})

	if expected != result {
		t.Errorf("Should have found %d invalid, found %d", expected, result)
	}

}

func TestSightCheck3(t *testing.T) {
	input := ReadFile("example4.txt")
	expected := 0
	result := SightCheck(input, 3, 3, map[string]bool{"#": true})

	if expected != result {
		t.Errorf("Should have found %d invalid, found %d", expected, result)
	}
}

func format(a [][]string) string {
	var s string
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[i]); j++ {
			s = s + a[i][j] + " "
		}
		s = s + "\n"
	}
	return s
}
