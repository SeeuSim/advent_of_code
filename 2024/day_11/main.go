package day11

import (
	"2024/utils"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(11, false)
	stones := GetGame(f)
	for i := 0; i < 25; i++ {
		var temp []int
		for _, stone := range stones {
			sFmt := strconv.Itoa(stone)
			var newStones []int
			if stone == 0 {
				newStones = append(newStones, 1)
			} else if len(sFmt)%2 == 0 {
				leftHalf, rightHalf := sFmt[:len(sFmt)/2], sFmt[len(sFmt)/2:]
				l, _ := strconv.Atoi(leftHalf)
				r, _ := strconv.Atoi(rightHalf)
				newStones = append(newStones, l, r)
			} else {
				newStones = append(newStones, stone*2024)
			}
			temp = append(temp, newStones...)
		}
		stones = temp
	}
	fmt.Printf("Score: %d\n", len(stones))
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetGame(f *os.File) []int {
	defer f.Close()
	var out []int
	c, _ := os.ReadFile(f.Name())
	for _, _ch := range strings.Fields(string(c)) {
		ch, _ := strconv.Atoi(_ch)
		out = append(out, ch)
	}
	return out
}
