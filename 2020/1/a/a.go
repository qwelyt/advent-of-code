package main;

import (
	"fmt"
	"bufio"
	"os"
	"log"
	"strconv"
);

type Pair struct {
	left, right int
}

func readFile(filePath string) []int{
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var values []int
	for scanner.Scan() {
		value,err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		values = append(values, value)
	}
	return values
}

func findPairThatSumsToValue(values []int, target int) Pair{
	for i :=0; i < len(values); i++ {
		for j := 0; j < len(values); j++ {
			if(i == j) {
				continue
			}
			if(values[i] + values[j] == target) {
				return Pair{values[i], values[j]}
			}
		}
	}
	return Pair{0,0}
}

func main(){
	values := readFile("../input.txt")
	//values := readFile("./example.txt")
	pair := findPairThatSumsToValue(values, 2020)
	multiply := pair.left * pair.right
	fmt.Println(multiply)
}