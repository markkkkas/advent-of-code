package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	calories := mustGroupAndSumEveryElveCalories()
	sort.Ints(calories)
	totalElves := len(calories)
	firstPart := calories[totalElves-1]
	secondPart := sum(calories[totalElves-3:])
	fmt.Println(firstPart, secondPart)
}

func mustReadInput(fileName string) string {
	f, err := os.ReadFile(fileName)
	if err != nil {
		panic(err)
	}
	return string(f)
}

func mustGroupAndSumEveryElveCalories() []int {
	var (
		input    = mustReadInput("./input.txt")
		scanner  = bufio.NewScanner(strings.NewReader(input))
		curr     = 0
		calories []int
	)
	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			curr += mustConvertStringToInt(line)
		} else {
			calories = append(calories, curr)
			curr = 0
		}
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return calories
}

func sum(nums []int) int {
	s := 0
	for _, n := range nums {
		s += n
	}
	return s
}

func mustConvertStringToInt(s string) int {
	val, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return val
}
