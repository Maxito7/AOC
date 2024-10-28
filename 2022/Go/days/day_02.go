package days

import "os"

func Day_02() {
	_, err := os.ReadFile("test_input.txt")
	if err != nil {
		panic(err)
	}
}
