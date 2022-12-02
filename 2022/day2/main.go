package main

import (
	"bufio"
	"fmt"
	"strings"

	"github.com/markkkkas/advent-of-code/utils"
)

var firstScores = map[string]int{
	"B X": 1,
	"C Y": 2,
	"A Z": 3,
	"A X": 4,
	"B Y": 5,
	"C Z": 6,
	"C X": 7,
	"A Y": 8,
	"B Z": 9,
}

var secondScores = map[string]int{
	"B X": 1,
	"C X": 2,
	"A X": 3,
	"A Y": 4,
	"B Y": 5,
	"C Y": 6,
	"C Z": 7,
	"A Z": 8,
	"B Z": 9,
}

func main() {
	var (
		input        = utils.MustReadInput("./input.txt")
		scanner      = bufio.NewScanner(strings.NewReader(input))
		firstPoints  = 0
		secondPoints = 0
	)
	for scanner.Scan() {
		line := scanner.Text()
		firstPoints += firstScores[line]
		secondPoints += secondScores[line]
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	fmt.Println(firstPoints, secondPoints)
}
