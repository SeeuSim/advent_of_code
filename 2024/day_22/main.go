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
	f := utils.OpenFile(22, false)
	g := GetGame(f)
	s := 0
	totals := make(map[ConsecChange]int)
	for _, n := range g {
		sec := n
		diffs := []int{}
		seen := make(map[ConsecChange]struct{})
		for i := 0; i < 2000; i++ {
			next := TransformNumber(sec)
			diffs = append(diffs, next%10-sec%10)
			sec = next
			if i >= 3 {
				id := ConsecChange{[4]int(diffs)}
				if _, s := seen[id]; !s {
					totals[id] += sec % 10
					seen[id] = struct{}{}
				}
				diffs = diffs[1:]
			}
		}
	}

	for _, v := range totals {
		if v > s {
			s = v
		}
	}
	fmt.Println("Sum:", s)
}

type ConsecChange struct {
	diffs [4]int
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
