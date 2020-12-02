package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Password struct {
	min    int
	max    int
	letter string
	str    string
}

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

func ProcessInput(rows []string) []Password {
	var passwords []Password
	for _, v := range rows {
		columns := strings.Split(v, " ")
		minmax := strings.Split(columns[0], "-")
		letter := strings.Split(columns[1], ":")
		min, minErr := strconv.Atoi(minmax[0])
		max, maxErr := strconv.Atoi(minmax[1])
		if minErr != nil {
			log.Fatal(minErr)
		}
		if maxErr != nil {
			log.Fatal(maxErr)
		}
		password := Password{min, max, letter[0], columns[2]}
		passwords = append(passwords, password)

	}
	return passwords
}

func ValidPasswords_partA(passwords []Password) (int, int) {
	var valid, invalid int

	for _, pass := range passwords {
		min := pass.min
		max := pass.max
		letter := pass.letter
		str := pass.str

		found := strings.Count(str, letter)
		if found < min || found > max {
			invalid++
		} else {
			valid++
		}
	}
	return valid, invalid
}

func ValidPasswords_partB(passwords []Password) (int, int) {
	var valid, invalid int

	for _, pass := range passwords {
		str := strings.Split(pass.str, "")
		first := str[pass.min-1]
		second := str[pass.max-1]

		if first == pass.letter && second == pass.letter {
			invalid++
		} else if first != pass.letter && second != pass.letter {
			invalid++
		} else {
			valid++
		}
	}
	return valid, invalid
}

func main() {
	values := ReadFile("input.txt")
	// values := ReadFile("example.txt")
	passwords := ProcessInput(values)
	validA, invalidA := ValidPasswords_partA(passwords)
	validB, invalidB := ValidPasswords_partB(passwords)
	fmt.Printf("=== Part A === \nValid: %d, Invalid: %d\n", validA, invalidA)
	fmt.Printf("=== Part B === \nValid: %d, Invalid: %d\n", validB, invalidB)
}
