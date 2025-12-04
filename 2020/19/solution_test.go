package main

import (
	"reflect"
	"testing"
)

func TestFindRuleChain(t *testing.T) {
	input := ReadFile("example.txt")
	result := FindRuleChain("0", input.rules)
	expected := []string{
		"ababbb",
		"abbbab",
	}
	if !reflect.DeepEqual(expected, result) {
		t.Errorf("\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}
