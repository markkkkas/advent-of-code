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
	input := readInput("./input.txt")
	firstPart(input)
	secondPart(input)
}

func readInput(fileName string) string {
	f, err := os.ReadFile(fileName)
	if err != nil {
		panic(err)
	}
	return string(f)
}

func firstPart(input string) {
	var (
		scanner = bufio.NewScanner(strings.NewReader(input))
		highest = 0
		curr    = 0
	)
	for scanner.Scan() {
		if scanner.Text() != "" {
			curr += stringToInt(scanner.Text())
		} else {
			if curr >= highest {
				highest = curr
			}

			curr = 0
		}
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	fmt.Println(highest)
}

func secondPart(input string) {
	var (
		scanner = bufio.NewScanner(strings.NewReader(input))
		curr    = 0
		kcals   []int
	)
	for scanner.Scan() {
		if scanner.Text() != "" {
			curr += stringToInt(scanner.Text())
		} else {
			kcals = append(kcals, curr)
			curr = 0
		}
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	sort.Ints(kcals)
	sum := 0
	for _, v := range kcals[len(kcals)-3:] {
		sum += v
	}
	fmt.Println(sum)
}

func stringToInt(s string) int {
	val, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return val
}
