package utils

import (
	"os"
	"strconv"
)

func MustReadInput(fileName string) string {
	f, err := os.ReadFile(fileName)
	if err != nil {
		panic(err)
	}
	return string(f)
}

func MustConvertStringToInt(s string) int {
	num, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return num
}

func IntsSum(ints []int) int {
	s := 0
	for _, n := range ints {
		s += n
	}
	return s
}
