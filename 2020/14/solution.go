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
	mask    bool
	memAddr int
	value   int64
}
type ProgMem struct {
	addr  int64
	value uint64
}

func (progMem *ProgMem) String() string {
	return fmt.Sprintf("(addr: %d, value: %d)", progMem.addr, progMem.value)
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

func ApplyMask(value string, mask []rune) uint64 {
	vInt, err := strconv.ParseUint(value, 10, 64)
	if err != nil {
		log.Fatal(err)
	}
	vBin := strconv.FormatUint(vInt, 2)
	zeros := strings.Repeat("0", len(mask)-len(vBin))
	sBin := zeros + vBin
	v := []rune(sBin)
	for i := 0; i < len(mask); i++ {
		if string(mask[i]) == "X" {
			continue
		}
		v[i] = mask[i]
	}

	dec, err := strconv.ParseUint(string(v), 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	return dec
}

func CreateAllFloating(runes []rune) []string {
	var addrs []string
	var mine string
	for i, v := range runes {
		if v == 'X' {
			if len(runes[i+1:]) == 0 {
				zero := mine + string("0")
				one := mine + string("1")
				addrs = append(addrs, zero)
				addrs = append(addrs, one)
			} else {
				subs := CreateAllFloating(runes[i+1:])
				for _, s := range subs {
					zero := mine + string("0") + s
					one := mine + string("1") + s
					addrs = append(addrs, zero)
					addrs = append(addrs, one)
				}
			}
		} else {
			mine += string(v)
		}
		if len(runes[i+1:]) == 0 {
			addrs = append(addrs, mine)
		}

	}
	return addrs
}

func StripShortOnes(length int, str []string) []string {
	var ret []string
	for _, s := range str {
		if len(s) == length {
			ret = append(ret, s)
		}
	}
	return ret
}

func ApplyMemMask(value string, mask []rune) []int64 {
	vInt, err := strconv.ParseInt(value, 10, 64)
	if err != nil {
		log.Fatal(err)
	}
	vBin := strconv.FormatInt(vInt, 2)
	zeros := strings.Repeat("0", len(mask)-len(vBin))
	sBin := zeros + vBin
	v := []rune(sBin)
	for i := 0; i < len(mask); i++ {
		if string(mask[i]) == "0" {
			continue
		} else if string(mask[i]) == "1" {
			v[i] = mask[i]
		} else if string(mask[i]) == "X" {
			v[i] = mask[i]
		}
	}
	addrStrs := CreateAllFloating(v)
	validAddr := StripShortOnes(len(mask), addrStrs)
	var mems []int64
	for _, a := range validAddr {
		i, err := strconv.ParseInt(a, 2, 64)
		if err != nil {
			log.Fatal(err)
		}
		mems = append(mems, i)
	}
	return mems

}

func ExtractMemAddr(str string) int {
	s := strings.Split(str, "[")
	s2 := strings.Split(s[1], "]")
	i, err := strconv.Atoi(s2[0])
	if err != nil {
		log.Fatal(err)
	}
	return i
}

func ProduceMemoryValues(instructions []string) []ProgMem {
	var mask []rune
	var memory []ProgMem
	for _, i := range instructions {
		is := strings.Split(i, " = ")
		if is[0] == "mask" {
			mask = []rune(is[1])
		} else {
			val := ApplyMask(is[1], mask)
			addr := ExtractMemAddr(is[0])
			memory = append(memory, ProgMem{int64(addr), val})
		}
	}
	return memory
}
func ProduceMemoryValues_Ver2(instructions []string) []ProgMem {
	var mask []rune
	var memory []ProgMem
	for _, i := range instructions {
		is := strings.Split(i, " = ")
		if is[0] == "mask" {
			mask = []rune(is[1])
		} else {
			val, err := strconv.Atoi(is[1])
			if err != nil {
				log.Fatal(err)
			}
			intAddr := ExtractMemAddr(is[0])
			binAddr := strconv.Itoa(intAddr)
			addr := ApplyMemMask(binAddr, mask)
			validAddrs := addr
			for _, a := range validAddrs {
				memory = append(memory, ProgMem{a, uint64(val)})
			}
		}
	}
	return memory
}

func ApplyMemory(progMem []ProgMem) []uint64 {
	var memPlaces int64
	for _, i := range progMem {
		if i.addr > memPlaces {
			memPlaces = i.addr
		}
	}
	memory := make([]uint64, memPlaces+1)
	for _, i := range progMem {
		memory[i.addr] = i.value
	}
	return memory
}

func SumMemory(progMem []ProgMem) uint64 {
	var sum uint64
	var memory = make(map[int64]uint64)

	for _, pm := range progMem {
		memory[pm.addr] = uint64(pm.value)
	}

	for _, v := range memory {
		sum += v
	}

	return sum
}

func PartA(instructions []string) uint64 {
	memValues := ProduceMemoryValues(instructions)
	memory := ApplyMemory(memValues)
	var sum uint64
	for _, m := range memory {
		sum += m
	}
	return sum
}

func PartB(instructions []string) uint64 {
	memValues := ProduceMemoryValues_Ver2(instructions)
	return SumMemory(memValues)
}

func main() {
	input := ReadFile("input.txt")
	fmt.Printf("=== Part A ===\nSum: %d\n", PartA(input))
	fmt.Printf("\n=== Part B ===\nSum: %d\n", PartB(input))
}
