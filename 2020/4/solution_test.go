package main

import "testing"

func TestPartB(t *testing.T) {
	input := ReadFile("example2.txt")
	valid, invalid, total := PartB(input)
	if valid != 4 || invalid != 4 || total != 8 {
		t.Errorf("Should get 4,4,8 but got: %d,%d,%d", valid, invalid, total)
	}
}

func TestValidValue(t *testing.T) {
	testField("byr", "2002", true, t)
	testField("byr", "2003", false, t)

	testField("hgt", "60in", true, t)
	testField("hgt", "190cm", true, t)
	testField("hgt", "190in", false, t)
	testField("hgt", "190", false, t)

	testField("hcl", "#123abc", true, t)
	testField("hcl", "#123abz", false, t)
	testField("hcl", "123abc", false, t)

	testField("ecl", "brn", true, t)
	testField("ecl", "wat", false, t)
	colours := []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
	for _, colour := range colours {
		testField("ecl", colour, true, t)
	}

	testField("pid", "000000001", true, t)
	testField("pid", "0123456789", false, t)
}

func testField(field string, value string, equals bool, t *testing.T) {
	if ValidValue(value, field) != equals {
		t.Errorf("%v %v did not equals %v", field, value, equals)
	}
}
