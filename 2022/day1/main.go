package main

import (
	"bufio"
	"fmt"
	"sort"
	"strings"

	"github.com/markkkkas/advent-of-code/utils"
)

func main() {
	calories := mustGroupAndSumEveryElveCalories()
	sort.Ints(calories)
	totalElves := len(calories)
	firstPart := calories[totalElves-1]
	secondPart := utils.IntsSum(calories[totalElves-3:])
	fmt.Println(firstPart, secondPart)
}

func mustGroupAndSumEveryElveCalories() []int {
	var (
		input    = utils.MustReadInput("./input.txt")
		scanner  = bufio.NewScanner(strings.NewReader(input))
		curr     = 0
		calories []int
	)
	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			curr += utils.MustConvertStringToInt(line)
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
