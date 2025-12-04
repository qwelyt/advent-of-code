package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Tuple2 struct {
	_1, _2 int
}
type Tuple3 struct {
	_1, _2, _3 int
}

func ReadFile(filePath string) []int {
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
		value, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		values = append(values, value)
	}
	return values
}

func SumTuple2(values []int, target int) Tuple2 {
	m := make(map[int]int)
	for _, v := range values {
		m[target-v] = v
	}

	for _, v := range m {
		other, ok := m[v]
		if ok {
			return Tuple2{v, other}
		}
	}
	return Tuple2{}

}
func remove(slice []int, index int) []int {
	return append(slice[:index], slice[index+1:]...)
}
func SumToTarget(values []int, target int) Tuple3 {
	for i := 0; i < len(values)-1; i++ {
		v := values[i]
		//arr := values
		//newValues := remove(arr, i)
		newTarget := target - v

		t2 := SumTuple2(values, newTarget)
		if v+t2._1+t2._2 == target {
			return Tuple3{t2._1, v, t2._2}
		}
	}

	// for i := 0; i < len(values); i++ {
	// 	for j := 0; j < len(values); j++ {
	// 		if i == j {
	// 			continue
	// 		}
	// 		for k := 0; k < len(values); k++ {
	// 			if i == k || j == k {
	// 				continue
	// 			}
	// 			if values[i]+values[j]+values[k] == target {
	// 				fmt.Printf("i: %d, j: %d, k: %d\n", i, j, k)
	// 				return Tuple3{values[i], values[j], values[k]}
	// 			}
	// 		}
	// 	}
	// }
	return Tuple3{}
}

func Multiply(tuple Tuple3) int {
	return tuple._1 * tuple._2 * tuple._3
}

func Debug() {
	main()
}

func main() {
	values := ReadFile("../input.txt")
	tuple := SumToTarget(values, 2020)
	fmt.Println(tuple)
	multiply := Multiply(tuple)
	fmt.Println(multiply)
}
