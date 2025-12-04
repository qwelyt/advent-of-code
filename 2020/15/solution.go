package main

import "fmt"

type Spoken struct {
	lastSpoken, mostRecent int
	hadBeenSpoken          bool
}

func PartA(input []int, spokenAt int) int {
	// RULES
	// 1: Speak all starting (input) numbers, they don't look at the previous spoken numbers.
	// 2: If the previous spoken number was the first time that number was spoken, say 0
	// 3: If the previous spoken number has been spoken before, take the previous turn number minus the time before that that that number
	//		was spoken. Speak the difference.

	var spoken = make(map[int][]int)
	var lastSpoken []int
	// Rule 1
	for i := 0; i < len(input); i++ {
		spoken[input[i]] = []int{i + 1}
		lastSpoken = append(lastSpoken, input[i])
	}

	for i := len(input) + 1; i < spokenAt+1; i++ {
		ls := lastSpoken[len(lastSpoken)-1] // Get the last spoken number
		th := spoken[ls]                    // Get all times that the last spoken number has been spoken
		if len(th) <= 1 {                   // Rule 2: The number has never been spoken OR the last time was the first time it was spoken
			lastSpoken = append(lastSpoken, 0)
			spoken[0] = append(spoken[0], i) // Since we spoke 0, append current turn to when 0 has been spoken
		} else { // Rule 3: The last spoken number has been spoken before
			previousTimes := th[len(th)-2:]
			diff := previousTimes[1] - previousTimes[0]
			lastSpoken = append(lastSpoken, diff)
			spoken[diff] = append(spoken[diff], i)
		}
	}

	return lastSpoken[len(lastSpoken)-1]
}

func main() {
	input := []int{16, 1, 0, 18, 12, 14, 19}
	fmt.Printf("=== Part A ===\n2020s number: %d\n", PartA(input, 2020))
	fmt.Printf("=== Part B ===\n30000000s number: %d\n", PartA(input, 30000000))
}
