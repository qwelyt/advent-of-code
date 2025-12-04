package main

import (
	"reflect"
	"testing"
)

func TestBagsInside(t *testing.T) {
	input := "2 posh gold, 3 dull green, 2 pale tan."
	expected := map[string]int{
		"posh gold":  2,
		"dull green": 3,
		"pale tan":   2}

	result := BagsInside(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}

func TestBagStructure(t *testing.T) {
	input := []string{"vibrant olive bags contain 2 posh turquoise bags, 3 mirrored fuchsia bags, 4 dotted beige bags."}
	expected := map[string]map[string]int{
		"vibrant olive": {
			"posh turquoise":   2,
			"mirrored fuchsia": 3,
			"dotted beige":     4}}
	result := BagStructure(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}
func TestBagStructure_example(t *testing.T) {
	input := ReadFile("example.txt")
	expected := map[string]map[string]int{
		"light red": {
			"bright white": 1,
			"muted yellow": 2},
		"dark orange": {
			"bright white": 3,
			"muted yellow": 4},
		"bright white": {
			"shiny gold": 1},
		"muted yellow": {
			"shiny gold": 2,
			"faded blue": 9},
		"shiny gold": {
			"dark olive":   1,
			"vibrant plum": 2},
		"dark olive": {
			"faded blue":   3,
			"dotted black": 4},
		"vibrant plum": {
			"faded blue":   5,
			"dotted black": 6},
		"faded blue":   {},
		"dotted black": {}}

	result := BagStructure(input)

	if !reflect.DeepEqual(expected, result) {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}

func TestPathTo(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 4
	paths := PathTo(BagStructure(input), "shiny gold")
	result := len(paths)

	if expected != result {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v\n%v", expected, result, paths)
	}
}
func TestCountBags1(t *testing.T) {
	input := ReadFile("example.txt")
	expected := 32
	result := CountBags(BagStructure(input), "shiny gold")

	if expected != result {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}
func TestCountBags2(t *testing.T) {
	input := ReadFile("example2.txt")
	expected := 126
	result := CountBags(BagStructure(input), "shiny gold")

	if expected != result {
		t.Errorf("Did not mach\nExpected:\t%v\nGot:\t\t%v", expected, result)
	}
}
