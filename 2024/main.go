package main

import (
	day01 "2024/day_01"
	day02 "2024/day_02"
	day03 "2024/day_03"
	day04 "2024/day_04"
	day05 "2024/day_05"
	day06 "2024/day_06"
	"fmt"
	"os"
	"strconv"
)

type G struct {
	RunP1 func()
	RunP2 func()
}

func main() {
	if len(os.Args) != 3 {
		fmt.Println("Usage: go run . <day> <part>")
		return
	}

	day := os.Args[1]
	part := os.Args[2]

	var days []G
	days = append(days, G{
		RunP1: day01.RunP1,
		RunP2: day01.RunP2,
	}, G{
		RunP1: day02.RunP1,
		RunP2: day02.RunP2,
	}, G{
		RunP1: day03.RunP1,
		RunP2: day03.RunP2,
	}, G{
		RunP1: day04.RunP1,
		RunP2: day04.RunP2,
	}, G{
		RunP1: day05.RunP1,
		RunP2: day05.RunP2,
	}, G{
		RunP1: day06.RunP1,
		RunP2: day06.RunP2,
	})

	d, e := strconv.Atoi(day)
	if e != nil {
		fmt.Printf("Invalid day: %s\n", day)
		return
	}
	if len(days) < d {
		fmt.Printf("Day %s not implemented yet\n", day)
		return
	}
	p, e := strconv.Atoi(part)
	if e != nil || p > 2 || p < 0 {
		fmt.Printf("Invalid part: %s\n", part)
		return
	}

	g := days[d-1]
	if p == 1 {
		g.RunP1()
	} else {
		g.RunP2()
	}
}
