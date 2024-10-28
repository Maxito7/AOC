package days

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func Day_01() {
	dat, err := os.ReadFile("/inputs/d1_input.in")

	check(err)
	content := string(dat)
	numbers := strings.Split(content, "\n\n")
	formatted_numbers := [][]string{}
	for i := 0; i < len(numbers); i++ {
		if numbers[i] != "" {
			formatted_numbers = append(formatted_numbers, strings.Split(numbers[i], "\n"))
		}
	}
	maxsum := 0
	first := 0
	second := 0
	third := 0
	for i := 0; i < len(formatted_numbers); i++ {
		sum := 0
		for j := 0; j < len(formatted_numbers[i]); j++ {
			value, _ := strconv.Atoi(formatted_numbers[i][j])
			sum += value
		}
		// NOTE: This conditional is for part A
		if sum >= maxsum {
			maxsum = sum
		}
		// NOTE: What's below is for part B
		if sum >= third {
			third = sum
		}
		if third >= second {
			third = second
			second = sum
		}
		if second >= first {
			second = first
			first = sum
		}
	}
	fmt.Println("Max calories:", maxsum)
	fmt.Println("Sum of top three:", first+second+third)
}
