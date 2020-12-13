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

type Bus struct {
	id, offset int
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
		a := t * id
		m[id] = a
	}
	return m
}

func BusArr(ins []string) []Bus {
	var m []Bus
	for i, v := range ins {
		id, err := strconv.Atoi(v)
		if err != nil {
			continue // x bus
		}
		m = append(m, Bus{id, i})
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

func Timestamp(buses []Bus) int {
	var lowest, highest int
	for _, k := range buses {
		if lowest == 0 || k.offset < lowest {
			lowest = k.offset
		}
		if highest == 0 || k.offset > highest {
			highest = k.offset
		}
	}
	for i := highest * lowest; ; i += lowest {
		b := true
		for _, v := range buses {
			if (i+v.offset)%v.id != 0 {
				b = false
			}
		}
		if b {
			return i
		}
	}
	return 0
}

func main() {
	input := ReadFile("input.txt")
	{
		buses := Buses(input)
		bus, time := Earliest(buses, input.arrival)
		t := time - input.arrival
		fmt.Printf("=== Part A ===\nBusId; %d, Time: %d, Wait: %d, Answer: %d\n\n", bus, time, t, t*bus)
	}

	{
		bus := BusArr(input.buses)
		timestamp := Timestamp(bus)
		fmt.Printf("=== Part B ===\nTimestamp: %d", timestamp)
	}
}
