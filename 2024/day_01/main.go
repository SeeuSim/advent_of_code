package day01

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func RunP1() {
	file, e := os.Open("./day_01/input.in")
	if e != nil {
		fmt.Printf("%s\n", e)
		return
	}
	defer file.Close()

	var lSlice, rSlice []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)
		if len(parts) != 2 {
			fmt.Printf("An error occurred: %s\n", line)
			return
		}

		num1, err := strconv.Atoi(parts[0])
		if err != nil {
			fmt.Printf("An error occurred: %s\n", parts[0])
			return
		}

		num2, err := strconv.Atoi(parts[1]) 
		if err != nil {
			fmt.Printf("An error occurred: %s\n", parts[1])
		}

		lSlice = append(lSlice, num1)
		rSlice = append(rSlice, num2)
	}

	sort.Ints(lSlice)
	sort.Ints(rSlice)

	sum := 0
	for i := range(lSlice) {
		sum += Abs(lSlice[i], rSlice[i])
	}
	fmt.Printf("SUM: %d\n", sum)
}

func RunP2() {
	file, e := os.Open("./day_01/input.in")
	if e != nil {
		fmt.Printf("%s\n", e)
		return
	}
	defer file.Close()

	rFreq := make(map[int]int)
	var lSlice []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)
		if len(parts) != 2 {
			fmt.Printf("An error occurred: %s\n", line)
			return
		}

		num1, err := strconv.Atoi(parts[0])
		if err != nil {
			fmt.Printf("An error occurred: %s\n", parts[0])
			return
		}

		num2, err := strconv.Atoi(parts[1]) 
		if err != nil {
			fmt.Printf("An error occurred: %s\n", parts[1])
		}

		lSlice = append(lSlice, num1)
		rFreq[num2] = rFreq[num2] + 1
	}
	sum := 0
	for i := range(lSlice) {
		sum += lSlice[i] * rFreq[lSlice[i]]
	}
	fmt.Printf("SUM: %d\n", sum)
}

func Abs(lNum, rNum int) int {
	if lNum < rNum {
		return rNum - lNum
	}
	return lNum - rNum
}