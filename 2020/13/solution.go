package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Instruction struct {
	arrival int
	buses   []string
}

func ReadFile(filePath string) Instruction {
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
		s := scanner.Text()
		values = append(values, s)
	}
	i, err := strconv.Atoi(values[0])
	if err != nil {
		log.Fatal(err)
	}
	return Instruction{i, strings.Split(values[1], ",")}
}

func Buses(ins Instruction) map[int]int {
	var m = make(map[int]int)

	for _, v := range ins.buses {
		id, err := strconv.Atoi(v)
		if err != nil {
			continue // x bus
		}
		var t int
		t = ins.arrival / id
		t += 1
		fmt.Println(t)
		a := t * id
		m[id] = a
	}
	return m
}

func Earliest(m map[int]int, arrival int) (int, int) {
	var id, time int
	time = 0
	for k, v := range m {
		if v < arrival {
			continue
		}
		if time == 0 || v < time {
			time = v
			id = k
		}

	}
	return id, time
}

func main() {
	input := ReadFile("input.txt")
	// input := ReadFile("exampel.txt")
	fmt.Println(input)
	buses := Buses(input)
	bus, time := Earliest(buses, input.arrival)
	fmt.Printf("%d, %d\n", bus, time)
	t := time - input.arrival
	fmt.Println(t)
	fmt.Println(t * bus)
}
