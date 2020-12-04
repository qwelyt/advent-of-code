package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func ReadFile(filePath string) []string {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values []string
	for scanner.Scan() {
		values = append(values, scanner.Text())
	}
	return values
}

func CreatePassport(rows []string) map[string]string {
	var m = make(map[string]string)
	for _, row := range rows {
		items := strings.Split(row, " ")
		for _, item := range items {
			kv := strings.Split(item, ":")
			m[kv[0]] = kv[1]
		}
	}
	return m
}

func ProcessData(input []string) []map[string]string {
	var rows []string
	var passports []map[string]string

	for i := 0; i < len(input); i++ {
		row := input[i]
		if len(row) == 0 {
			passports = append(passports, CreatePassport(rows))
			rows = nil
		} else {
			rows = append(rows, row)
		}
	}
	if len(rows) > 0 {
		passports = append(passports, CreatePassport(rows))
	}
	return passports
}

func ValidPassportsNoValidation(passports []map[string]string, mustHaveFields []string) (int, int, int) {
	var valid, invalid int
	for _, p := range passports {
		for _, f := range mustHaveFields {
			_, ok := p[f]
			if !ok {
				invalid++
				break
			}
		}
	}
	valid = len(passports) - invalid
	return valid, invalid, len(passports)
}

func ValidValue(value string, field string) bool {
	switch field {
	case "byr":
		num, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		if num >= 1920 && num <= 2002 {
			return true
		}
	case "iyr":
		num, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		if num >= 2010 && num <= 2020 {
			return true
		}
	case "eyr":
		num, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		if num >= 2020 && num <= 2030 {
			return true
		}
	case "hgt":
		h := string(value[len(value)-2:])
		num, err := strconv.Atoi(value[:len(value)-2])
		if err != nil {
			return false
		}
		if h == "cm" {
			if num >= 150 && num <= 193 {
				return true
			}
		} else if h == "in" {
			if num >= 59 && num <= 76 {
				return true
			}
		}
	case "hcl":
		if string(value[0]) == "#" {
			if len(value[1:]) == 6 {
				_, err := strconv.ParseUint(value[1:], 16, 64)
				if err == nil {
					return true
				}
			}
		}
	case "ecl":
		switch value {
		case "amb":
			return true
		case "blu":
			return true
		case "brn":
			return true
		case "grn":
			return true
		case "gry":
			return true
		case "hzl":
			return true
		case "oth":
			return true
		}
	case "pid":
		if len(value) == 9 {
			num, err := strconv.Atoi(value)
			if err != nil {
				return false
			}
			if num >= 0 && num <= 999999999 {
				return true
			}
		}
	case "cid":
		return true
	}
	return false
}

func ValidPassportsWithValidation(passports []map[string]string, mustHaveFields []string) (int, int, int) {
	var valid, invalid int
	for _, p := range passports {
		for _, f := range mustHaveFields {
			value, present := p[f]
			if !present || !ValidValue(value, f) {
				// fmt.Printf("== Invalid\n passport: %v\n value %v, field %v\n", p, value, f)
				invalid++
				break
			}
		}
	}
	valid = len(passports) - invalid
	return valid, invalid, len(passports)
}

func PartA(input []string) (int, int, int) {
	mustHaveFields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	passports := ProcessData(input)
	valid, invalid, total := ValidPassportsNoValidation(passports, mustHaveFields)
	return valid, invalid, total
}

func PartB(input []string) (int, int, int) {
	mustHaveFields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	passports := ProcessData(input)
	valid, invalid, total := ValidPassportsWithValidation(passports, mustHaveFields)
	return valid, invalid, total
}

func execute(part string, input []string, fn func([]string) (int, int, int)) {
	valid, invalid, total := fn(input)
	fmt.Printf("=== %v ===\nValid: %d, Invalid: %d, total: %d\n", part, valid, invalid, total)
}

func main() {
	// input := ReadFile("example.txt")
	// input := ReadFile("example2.txt")
	input := ReadFile("input.txt")
	execute("Part A", input, PartA)
	execute("Part B", input, PartB)
}
