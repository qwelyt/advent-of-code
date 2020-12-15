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
	var lastSpoken int
	// Rule 1
	for i := 0; i < len(input); i++ {
		spoken[input[i]] = []int{i + 1}
		lastSpoken = input[i]
	}

	for i := len(input) + 1; i < spokenAt+1; i++ {
		th := spoken[lastSpoken] // Get all times that the last spoken number has been spoken
		if len(th) <= 1 {        // Rule 2: The number has never been spoken OR the last time was the first time it was spoken
			lastSpoken = 0
			spoken[0] = append(spoken[0], i) // Since we spoke 0, append current turn to when 0 has been spoken
		} else { // Rule 3: The last spoken number has been spoken before
			previousTimes := th[len(th)-2:]
			diff := previousTimes[1] - previousTimes[0]
			lastSpoken = diff
			spoken[diff] = append(spoken[diff], i)
			if len(spoken[diff]) > 2 {
				spoken[diff] = spoken[diff][len(spoken[diff])-2:]
			}
		}
	}

	return lastSpoken
}

func PartAChannels(input []int, spokenAt int) int {
	// RULES
	// 1: Speak all starting (input) numbers, they don't look at the previous spoken numbers.
	// 2: If the previous spoken number was the first time that number was spoken, say 0
	// 3: If the previous spoken number has been spoken before, take the previous turn number minus the time before that that that number
	//		was spoken. Speak the difference.

	var spoken = make(map[int](chan int))
	var lastSpoken int
	// Rule 1
	for i := 0; i < len(input); i++ {
		spoken[input[i]] = make(chan int, 2)
		spoken[input[i]] <- i + 1
		lastSpoken = input[i]
	}

	for i := len(input) + 1; i < spokenAt+1; i++ {
		th := spoken[lastSpoken] // Get all times that the last spoken number has been spoken
		if len(th) <= 1 {        // Rule 2: The number has never been spoken OR the last time was the first time it was spoken
			lastSpoken = 0
			if len(spoken[0]) == 2 {
				<-spoken[0]
			}
			spoken[0] <- i // Since we spoke 0, append current turn to when 0 has been spoken
		} else { // Rule 3: The last spoken number has been spoken before
			first := <-th
			second := <-th
			diff := second - first
			if spoken[diff] == nil {
				spoken[diff] = make(chan int, 2)
			}
			if diff != lastSpoken {
				if len(spoken[diff]) == 2 {
					<-spoken[diff]
				}
				spoken[diff] <- i
				spoken[lastSpoken] <- first // Put 'em back
				spoken[lastSpoken] <- second
			} else {
				spoken[diff] <- second
				spoken[diff] <- i
			}
			lastSpoken = diff
		}
	}
	return lastSpoken
}
func main() {
	input := []int{16, 1, 0, 18, 12, 14, 19}
	fmt.Printf("=== Part A ===\n2020s number: %d\n", PartA(input, 2020))
	fmt.Printf("=== Part B ===\n30000000s number: %d\n", PartA(input, 30000000))
}
