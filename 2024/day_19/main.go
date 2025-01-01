package day19

import (
	"2024/utils"
	"fmt"
	"os"
	"strings"
)

type RecursiveFnArgs struct {
	towel, design string
}

func RecursiveSearch(towel, design string, seen map[RecursiveFnArgs]bool, towels []string) bool {
	if r, e := seen[RecursiveFnArgs{towel, design}]; e {
		return r
	}
	out := false
	out = out || design == ""
	if !out {
		for _, t := range towels {
			if strings.HasPrefix(design, t) && RecursiveSearch(t, design[len(t):], seen, towels) {
				out = true
				break
			}
		}
	}
	seen[RecursiveFnArgs{towel, design}] = out
	return out
}

func RunP1() {
	f := utils.OpenFile(19, false)
	towels, designs := GetGame(f)
	seen := make(map[RecursiveFnArgs]bool)

	s := 0
	for _, design := range designs {
		isPossible := false
		for _, towel := range towels {
			isPossible = isPossible || RecursiveSearch(towel, design, seen, towels)
			if isPossible {
				break
			}
		}
		if isPossible {
			s++
		}
	}
	fmt.Println("Sum:", s)
}

func RecursiveSearchP2(towel, design string, seen map[RecursiveFnArgs]int, towels []string) int {
	args := RecursiveFnArgs{towel, design}
	if r, e := seen[args]; e {
		return r
	}
	if design == "" {
		seen[args] = 1
		return 1
	}
	combinations := 0
	for _, t := range towels {
		if strings.HasPrefix(design, t) {
			combinations += RecursiveSearchP2(t, design[len(t):], seen, towels)
		}
	}
	seen[args] = combinations
	return combinations
}

func RunP2() {
	f := utils.OpenFile(19, true)
	towels, designs := GetGame(f)

	s := 0
	for _, d := range designs {
		seen := make(map[RecursiveFnArgs]int)
		c := 0
		for _, t := range towels {
			c += RecursiveSearchP2(t, d, seen, towels)
		}
		c /= len(towels)
		s += c
	}
	fmt.Println("Sum of ways:", s)
}

func GetGame(f *os.File) ([]string, []string) {
	defer f.Close()
	_content, _ := os.ReadFile(f.Name())
	content := string(_content)

	p := strings.Split(content, "\n\n")
	_towels, _designs := p[0], p[1]
	towels := strings.Split(_towels, ", ")
	designs := strings.Split(_designs, "\n")

	return towels, designs
}
