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
	f := utils.OpenFile(11, false)
	stones := GetGame(f)
	cache := make(map[Args]int)
	score := 0
	for _, s := range stones {
		score += GetNStones(s, 75, cache)
	}
	fmt.Printf("Score: %d\n", score)
}

type Args struct {
	stone  int
	blinks int
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

func GetNStones(stone int, blinks int, cache map[Args]int) int {
	if blinks == 0 {
		return 1
	}
	arg := Args{stone, blinks}
	if v, exists := cache[arg]; exists {
		return v
	}
	res := 0
	if stone == 0 {
		res = GetNStones(1, blinks-1, cache)
	} else {
		sFmt := strconv.Itoa(stone)
		if len(sFmt)%2 == 0 {
			l, _ := strconv.Atoi(sFmt[:len(sFmt)/2])
			r, _ := strconv.Atoi(sFmt[len(sFmt)/2:])
			res = GetNStones(l, blinks-1, cache) + GetNStones(r, blinks-1, cache)
		} else {
			res = GetNStones(stone*2024, blinks-1, cache)
		}
	}
	cache[arg] = res
	return res
}
