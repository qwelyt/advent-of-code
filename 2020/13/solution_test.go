package main

import "testing"

func TestTimestamp(t *testing.T) {
	input := []string{"7", "13", "x", "x", "59", "x", "31", "19"}
	result := Timestamp(BusArr(input))
	expected := 1068781

	if expected != result {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestTimestampMulti(t *testing.T) {
	var cases = make(map[int][]string)
	cases[3417] = []string{"17", "x", "13", "19"}
	cases[754018] = []string{"67", "7", "59", "61"}
	cases[779210] = []string{"67", "x", "7", "59", "61"}
	cases[1261476] = []string{"67", "7", "x", "59", "61"}
	cases[1202161486] = []string{"1789", "37", "47", "1889"}

	for expected, input := range cases {
		result := Timestamp(BusArr(input))
		if expected != result {
			t.Errorf("Expected %d, got %d\t\tInput: %v", expected, result, input)
		}

	}
}
