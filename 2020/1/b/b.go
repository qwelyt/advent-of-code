package main;

import (
	"fmt"
	"bufio"
	"os"
	"log"
	"strconv"
);

type Tuple3 struct {
	_1,_2,_3 int
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

func sumToTarget(values []int, target int) Tuple3{
	for i := 0; i < len(values); i++ {
		for j := 0; j < len(values); j++ {
			if i == j {
				continue
			}
			for k := 0; k < len(values); k++ {
				if i == k || j == k {
					continue
				}
				if values[i] + values[j] + values[k] == target {
					return Tuple3{values[i], values[j], values[k]}
				}
			}
		}
	}
	return Tuple3{}
}

func main(){
	values := readFile("../input.txt")
	//values := readFile("../example.txt")
	tuple := sumToTarget(values, 2020)
	multiply := tuple._1 * tuple._2 * tuple._3
	fmt.Println(multiply)
}