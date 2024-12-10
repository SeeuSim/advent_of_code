package day02

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func RunP1() {
	file := OpenInput()
	defer file.Close()

	scanner := bufio.NewScanner(file)
	nSafe := 0
	for scanner.Scan() {
		report := scanner.Text()
		levels := GetLevels(report)
		isSafe := IsSafeReport(levels)
		if isSafe {
			nSafe += 1
		}
	}
	fmt.Printf("Safe Reports: %d\n", nSafe)
}

func RunP2() {
	file := OpenInput()
	defer file.Close()

	scanner := bufio.NewScanner(file)
	nSafe := 0
	for scanner.Scan() {
		report := scanner.Text()
		levels := GetLevels(report)
		isSafe := IsSafeReport(levels)
		if isSafe || IsSafeByRemovingOne(levels) {
			nSafe += 1
		}
	}
	fmt.Printf("Safe Reports: %d\n", nSafe)
}

func OpenInput() *os.File {
	f, e := os.Open("./day_02/input.in")
	if e != nil {
		fmt.Printf("Error opening input file: %s\n", e)
		os.Exit(1)
		return nil
	}
	return f
}

func GetLevels(report string) []int {
	var out []int
	levels := strings.Fields(report)
	for i := range levels {
		v, _ := strconv.Atoi(levels[i])
		out = append(out, v)
	}
	return out
}

func IsSafeReport(levels []int) bool {
	prev := 0
	isIncreasing := false
	for i := range levels {
		v := levels[i]
		if i == 0 {
			prev = v
			continue
		}

		diff := Abs(prev, v)
		if diff == 0 || diff > 3 {
			return false
		}
		if i == 1 {
			isIncreasing = prev < v
			prev = v
			continue
		}

		if (isIncreasing && prev > v) || (!isIncreasing && prev < v) {
			return false
		}
		prev = v
	}
	return true
}

func IsSafeByRemovingOne(_levels []int) bool {
	for i := 0; i < len(_levels); i++ {
		levels := make([]int, len(_levels))
		copy(levels, _levels[:])
		slice := append(levels[:i], levels[i+1:]...)
		if IsSafeReport(slice) {
			return true
		}
	}
	return false
}

func Abs(l, r int) int {
	o := l - r
	if o < 0 {
		return -o
	}
	return o
}
