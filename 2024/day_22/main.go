package day22

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func RunP1() {
	f := utils.OpenFile(22, false)
	g := GetGame(f)
	s := 0
	for _, n := range g {
		sec := n
		for i := 0; i < 2000; i++ {
			sec = TransformNumber(sec)
		}
		s += sec
	}
	fmt.Println("Sum:", s)
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetGame(f *os.File) []int {
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out []int
	for scanner.Scan() {
		l := scanner.Text()
		n, _ := strconv.Atoi(l)
		out = append(out, n)
	}
	return out
}

func TransformNumber(number int) int {
	result := number * 64
	out := Mix(result, number)
	out = Prune(out)

	result = out / 32
	out = Mix(result, out)
	out = Prune(out)

	result = out * 2048
	out = Mix(result, out)
	out = Prune(out)

	return out
}

func Mix(res, number int) int {
	return res ^ number
}

func Prune(number int) int {
	return number % 16777216
}
