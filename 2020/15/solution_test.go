package main

import "testing"

func TestPartA(t *testing.T) {
	input := []int{0, 3, 6}
	expected := 436
	result := PartA(input, 2020)

	if expected != result {
		t.Errorf("Expected: %d, Got: %d", expected, result)
	}
}

func TestPartAExamples(t *testing.T) {
	var cases = map[int][]int{
		436:  {0, 3, 6},
		1:    {1, 3, 2},
		10:   {2, 1, 3},
		27:   {1, 2, 3},
		78:   {2, 3, 1},
		438:  {3, 2, 1},
		1836: {3, 1, 2},
	}

	for expected, input := range cases {
		result := PartA(input, 2020)

		if expected != result {
			t.Errorf("Expected: %d, Got: %d", expected, result)
		}
	}
}

func TestPartAExamples_channels(t *testing.T) {
	var cases = map[int][]int{
		436:  {0, 3, 6},
		1:    {1, 3, 2},
		10:   {2, 1, 3},
		27:   {1, 2, 3},
		78:   {2, 3, 1},
		438:  {3, 2, 1},
		1836: {3, 1, 2},
	}

	for expected, input := range cases {
		result := PartAChannels(input, 2020)

		if expected != result {
			t.Errorf("Expected: %d, Got: %d", expected, result)
		}
	}
}

func TestPartBExamples(t *testing.T) {
	var cases = map[int][]int{

		175594:  {0, 3, 6},
		2578:    {1, 3, 2},
		3544142: {2, 1, 3},
		261214:  {1, 2, 3},
		6895259: {2, 3, 1},
		18:      {3, 2, 1},
		362:     {3, 1, 2},
	}

	for expected, input := range cases {
		result := PartA(input, 30000000)

		if expected != result {
			t.Errorf("Expected: %d, Got: %d", expected, result)
		}
	}
}

func TestSolve(t *testing.T) {
	input := []int{16, 1, 0, 18, 12, 14, 19}
	partA := PartA(input, 2020)
	if partA != 929 {
		t.Errorf("Part A broke. Got %d", partA)
	}
	partB := PartA(input, 30000000)
	if partB != 16671510 {
		t.Errorf("Part B broke. Got %d", partB)
	}

}
