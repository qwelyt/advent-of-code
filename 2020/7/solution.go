package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Path struct {
	colour string
	cost   int
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

func BagsInside(bagLine string) map[string]int {
	var m = make(map[string]int)

	bags := strings.Split(strings.Replace(bagLine, ".", "", -1), ", ")
	for _, bag := range bags {
		b := strings.Split(bag, " ")
		n, err := strconv.Atoi(bag[0:len(b[0])])
		if err != nil {
			fmt.Printf("Could not parse %v\n", bag)
		}
		k := bag[len(b[0])+1 : len(bag)]
		m[k] = n
	}
	return m
}

func BagStructure(input []string) map[string]map[string]int {
	var m = make(map[string]map[string]int)

	for _, row := range input {
		cleanedRow := strings.Replace(strings.Replace(row, " bags", "", -1), " bag", "", -1)
		rs := strings.Split(cleanedRow, " contain ")
		k := rs[0]
		if rs[1] == "no other" || rs[1] == "no other." {
			m[k] = make(map[string]int)
		} else {
			m[k] = BagsInside(rs[1])
		}
	}
	return m
}

func traverse(curr string, rules map[string]map[string]int, find string) (bool, []string) {
	var path []string
	lookAt := rules[curr]
	if _, ok := lookAt[find]; ok {
		path = append(path, curr)
		return true, path
	}
	for k, _ := range lookAt {
		if k == find {
			continue
		}
		present, p := traverse(k, rules, find)
		path = append(path, p...)
		if present {
			return true, path
		}

	}
	return false, path
}

func travSub(curr string, rules map[string]map[string]int) []string {
	path := []string{curr}
	if len(rules[curr]) == 0 {
		return path
	}
	for k := range rules[curr] {
		path = append(path, travSub(k, rules)...)
	}
	return path
}

func trav(curr string, rules map[string]map[string]int) [][]string {
	var paths [][]string
	top := rules[curr]
	for k, _ := range top {
		b := travSub(k, rules)
		paths = append(paths, b)
	}
	return paths
}

func trol(curr string, rules map[string]map[string]int) (int, []string) {
	if len(rules[curr]) == 0 {
		return 1, []string{curr}
	}
	var p int
	var path []string
	for k, v := range rules[curr] {
		cost, pat := trol(k, rules)
		path = append(path, pat...)
		p += v
		p += cost
	}
	return p, path
}

func tral(curr string, rules map[string]map[string]int) (int, [][]string) {
	var paths [][]string
	var c int
	top := rules[curr]
	for k, v := range top {
		costs, pat := trol(k, rules)
		paths = append(paths, pat)
		c += v
		c += costs
	}
	return c, paths
}

func traverse2(curr string, rules map[string]map[string]int) (bool, []string) {
	var path []string
	lookAt := rules[curr]
	if len(lookAt) == 0 {
		path = append(path, curr)
		return true, path
	}
	for k, _ := range lookAt {
		present, p := traverse2(k, rules)
		if present {
			path = append(path, k)
			path = append(path, p...)
			return true, path
		}

	}
	return false, path
}

func PathTo(rules map[string]map[string]int, find string) [][]string {
	var paths [][]string
	for k, _ := range rules {
		present, path := traverse(k, rules, find)
		if present {
			paths = append(paths, path)
		}
	}
	return paths
}

func CountBags(rules map[string]map[string]int, find string) int {
	// var count int
	// var journey = []string{}
	// var costs []int

	// fmt.Println(trav(find, rules))
	// fmt.Println("===")
	p, q := tral(find, rules)
	for _, v := range q {
		p += len(v)
	}
	p += len(q) + 1
	// fmt.Println(tral(find, rules))
	// fmt.Println(p, q)
	// fmt.Println("===")
	return p

	// var custs [][]Path
	// travs := trav(find, rules)
	// for _, v := range travs {
	// 	var cs []Path
	// 	for i := 0; i < len(v); i++ {
	// 		if len(rules[v[i]]) == 0 {
	// 			costs = append(costs, 1)
	// 			continue
	// 		}
	// 		if i+1 == len(v) {
	// 			continue
	// 		}
	// 		c := rules[v[i]][v[i+1]]
	// 		cs = append(cs, Path{v[i] + "::" + v[i+1], c})
	// 	}
	// 	custs = append(custs, cs)
	// }
	// fmt.Println(custs)

	// // _, path := traverse2(find, rules)
	// // // path := trav(find, rules)
	// // journey = append(journey, path...)

	// // for i := 0; i < len(journey); i++ {
	// // 	if i+1 == len(journey) {
	// // 		costs = append(costs, 1)
	// // 		continue
	// // 	}
	// // 	c := rules[journey[i]][journey[i+1]]
	// // 	costs = append(costs, c)
	// // }

	// for i := 0; i < len(costs); i++ {
	// 	if i+1 == len(costs) {
	// 		count++
	// 		continue
	// 	}
	// 	count += costs[i]
	// 	count += costs[i] * costs[i+1]
	// }

	// fmt.Println(costs)

	// return count
}

func main() {
	input := ReadFile("input.txt")
	bagStructure := BagStructure(input)
	canContain := PathTo(bagStructure, "shiny gold")
	fmt.Printf("=== Part A ===\nPossible structures: %d\n", len(canContain))

	fmt.Println(CountBags(bagStructure, "shiny gold"))

}
