package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Input struct {
	rules          map[string]map[int]bool
	flattenedRules map[int]bool
	myTicket       []int
	nearbyTickets  [][]int
}

func ParseRules(rules []string) map[string]map[int]bool {
	var m = make(map[string]map[int]bool)
	for _, s := range rules {
		kv := strings.Split(s, ": ")
		key := kv[0]

		var values = make(map[int]bool)
		vs := strings.Split(kv[1], " or ")
		for _, v := range vs {
			minmax := strings.Split(v, "-")
			min, err := strconv.Atoi(minmax[0])
			if err != nil {
				log.Fatal(err)
			}
			max, err := strconv.Atoi(minmax[1])
			if err != nil {
				log.Fatal(err)
			}

			for i := min; i < max+1; i++ {
				values[i] = true
			}
		}
		m[key] = values
	}
	return m
}

func FlattenRules(rules map[string]map[int]bool) map[int]bool {
	var m = make(map[int]bool)
	for _, innerMap := range rules {
		for i, b := range innerMap {
			m[i] = b
		}
	}

	return m
}

func ReadFile(filePath string) Input {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var rules []string
	var myTicket []int
	var tickets [][]int
	var workOn int
	for scanner.Scan() {
		s := scanner.Text()
		if len(s) == 0 {
			continue
		}
		switch s[0] {
		case 'c':
			fallthrough
		case 'r':
			fallthrough
		case 's':
			workOn = 0 // Rules
		case 'y':
			workOn = 1 // My ticket
			continue
		case 'n':
			workOn = 2 // Nearby tickets
			continue
		}

		switch workOn {
		case 0:
			rules = append(rules, s)
		case 1:
			spl := strings.Split(s, ",")
			for _, sp := range spl {
				i, err := strconv.Atoi(sp)
				if err != nil {
					log.Fatal(err)
				}
				myTicket = append(myTicket, i)
			}
		case 2:
			spl := strings.Split(s, ",")
			var tick []int
			for _, sp := range spl {
				i, err := strconv.Atoi(sp)
				if err != nil {
					log.Fatal(err)
				}
				tick = append(tick, i)
			}
			tickets = append(tickets, tick)
		}
	}
	parsedRules := ParseRules(rules)
	flattenedRules := FlattenRules(parsedRules)
	return Input{parsedRules, flattenedRules, myTicket, tickets}
}

func ErrorsInNearbyTickets(input Input) []int {
	var errors []int
	for _, t := range input.nearbyTickets { // t == []int
		for _, i := range t {
			if !input.flattenedRules[i] {
				errors = append(errors, i)
			}
		}
	}
	return errors
}

func SumErrors(errs []int) int {
	var sum int
	for _, i := range errs {
		sum += i
	}
	return sum
}

func RemoveInvalidTickets(input Input) Input {
	var tickets [][]int

	for _, t := range input.nearbyTickets { // t == []int
		add := true
		for _, i := range t {
			if !input.flattenedRules[i] {
				add = false
			}
		}
		if add {
			tickets = append(tickets, t)
		}
	}

	return Input{input.rules, input.flattenedRules, input.myTicket, tickets}
}

type Span struct {
	min, max int
}

func CorrelateFields(input Input) map[int]string {
	var columnValues = make(map[int][]int)
	for _, ticket := range input.nearbyTickets {
		for column, value := range ticket {
			columnValues[column] = append(columnValues[column], value)
		}
	}

	var possibleFits = make(map[string][]int)
	for ruleName, possibleValuesInRule := range input.rules {
		for column, columnValue := range columnValues {
			canFitAll := true
			for _, value := range columnValue {
				if !possibleValuesInRule[value] {
					canFitAll = false
					break
				}
			}
			if canFitAll {
				possibleFits[ruleName] = append(possibleFits[ruleName], column)
			}
		}

	}
	invert := Invert(possibleFits)
	uc, c := CleanFit(invert, map[int]string{})
	for len(uc) > 0 {
		uc, c = CleanFit(uc, c)
	}

	return c
}

func Invert(m map[string][]int) map[int][]string {
	var w = make(map[int][]string)
	for k, v := range m {
		for _, i := range v {
			w[i] = append(w[i], k)
		}
	}
	return w
}

func Remove(fromThis []string, removeThese map[int]string) []string {
	var m = make(map[string]bool)
	var ret []string
	for _, rt := range removeThese {
		m[rt] = true
	}
	for _, value := range fromThis {
		if !m[value] {
			ret = append(ret, value)
		}
	}
	return ret
}

func CleanFit(unclean map[int][]string, clean map[int]string) (map[int][]string, map[int]string) {
	var newUnclean = make(map[int][]string)
	for k, v := range unclean {
		uniq := Remove(v, clean)
		if len(uniq) == 1 {
			clean[k] = uniq[0]
		} else {
			newUnclean[k] = uniq
		}
	}
	return newUnclean, clean
}

func MultiplyDeparture(input Input, columns map[int]string) uint64 {
	var sum uint64
	sum = 1
	for k, v := range columns {
		if len(v) > 8 && string(v[:9]) == "departure" {
			sum *= uint64(input.myTicket[k])
		}
	}
	return sum
}

func main() {
	// input := ReadFile("example.txt")
	input := ReadFile("input.txt")
	{
		errors := ErrorsInNearbyTickets(input)
		sumOfErrors := SumErrors(errors)
		fmt.Printf("=== Part A ===\nError rate: %d\n", sumOfErrors)
	}
	{
		clean := RemoveInvalidTickets(input)
		columns := CorrelateFields(clean)
		sum := MultiplyDeparture(clean, columns)
		fmt.Printf("=== Part B ===\nDeparture sum: %d\n", sum)
	}
}
