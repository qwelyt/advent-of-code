package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Input struct {
	rules    map[string][][]string
	messages []string
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
	var messages []string
	rulesDone := false
	for scanner.Scan() {
		str := scanner.Text()
		if str == "" || len(str) == 0 {
			rulesDone = true
			continue
		}
		if rulesDone {
			messages = append(messages, str)
		} else {
			rules = append(rules, str)
		}
	}

	return Input{FixRules(rules), messages}
}

func FixRules(rules []string) map[string][][]string {
	var m = make(map[string][][]string)

	for _, rule := range rules {
		kv := strings.Split(rule, ": ")
		v := strings.Split(kv[1], " | ")
		var values [][]string
		for _, value := range v {
			cleanValue := strings.ReplaceAll(value, "\"", "")
			values = append(values, strings.Split(cleanValue, " "))
		}
		m[kv[0]] = values
	}

	return m
}

func FindRuleChain(find string, rules map[string][][]string) []string {
	/*
		0: 1 2 | 3 4
		1: 2
		2: b
		3: 2 2 2 5
		4: 1 3 1
		5: a

		find 0
		1 2 -> 1(2(b)) 2(b) -> b b
		3 4 -> 3(2(b) 2(b) 2(b) 5(a))  4(1(2(b)) 3(2(b) 2(b) 2(b) 5(a))) -> b b b a b b b a

		1) Start by finding *this* rule
		2) For every possible value (1 2 | 3 4) traverse down the rule chain and find the end (a or b)
		3) Build up the possible string for each path
		If a rule has a fork (|) it results in two variations.
		The rusulting possible rules could be *a lot* as each fork in each rule gives two paths.
	*/
	theseRules := rules[find] // 1
	var out []string

	for _, rule := range theseRules { // 2, rule == []string
		if rule[0] == "a" || rule[0] == "b" { // We reached the end.
			out = append(out, rule[0])
		} else { // Continue traverse down
			var str []string
			for _, s := range rule { // s == string (so "1", or "2" etc)
				var stina string
				strs := FindRuleChain(s, rules)
				for _, st := range strs {
					stina += st
				}
				str = append(str, stina)
			}
			out = append(out, str...)
		}
	}

	return out
}

func main() {
	input := ReadFile("input.txt")
	fmt.Println(input)
}
