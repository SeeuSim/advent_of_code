package main

import (
	day01 "2024/day_01"
	day02 "2024/day_02"
	day03 "2024/day_03"
	day04 "2024/day_04"
	day05 "2024/day_05"
	day06 "2024/day_06"
	day07 "2024/day_07"
	day08 "2024/day_08"
	day09 "2024/day_09"
	day10 "2024/day_10"
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
	days = append(days,
		G{day01.RunP1, day01.RunP2},
		G{day02.RunP1, day02.RunP2},
		G{day03.RunP1, day03.RunP2},
		G{day04.RunP1, day04.RunP2},
		G{day05.RunP1, day05.RunP2},
		G{day06.RunP1, day06.RunP2},
		G{day07.RunP1, day07.RunP2},
		G{day08.RunP1, day08.RunP2},
		G{day09.RunP1, day09.RunP2},
		G{day10.RunP1, day10.RunP2},
	)

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
