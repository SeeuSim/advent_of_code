package day05

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(5, false)
	defer f.Close()

	// Given X | Y, X has to appear before Y
	rules, updates := GetData(f)
	sum := 0
	for _i := range updates {
		update := updates[_i]
		isCorrect := true
		for i := 1; i < len(update); i++ {
			if !isCorrect {
				break
			}
			curr := update[i]
			for j := 0; j < i; j++ {
				// For each prior element, check if i-th element is supposed to appear before
				if !isCorrect {
					break
				}
				prior := update[j]
				priorRules := rules[prior]
				for ruleIdx := range priorRules {
					if priorRules[ruleIdx] == curr {
						// illegal
						isCorrect = false
						break
					}
				}
			}
		}
		if isCorrect {
			sum += update[len(update)/2]
		}
	}
	fmt.Printf("Sum: %d\n", sum)
}

func RunP2() {

}

func GetData(f *os.File) (map[int][]int, [][]int) {
	scanner := bufio.NewScanner(f)

	ruleRegex := regexp.MustCompile(`^(\d+)\|(\d+)$`)
	// Store [key][*before]
	rules := make(map[int][]int)
	var updates [][]int
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		if ruleRegex.MatchString(line) {
			// Process rule
			parts := strings.Split(line, "|")
			_x, _y := parts[0], parts[1]
			x, _ := strconv.Atoi(_x)
			y, _ := strconv.Atoi(_y)
			rules[y] = append(rules[y], x)

		} else {
			parts := strings.Split(line, ",")
			var row []int
			for i := range parts {
				_p := parts[i]
				p, _ := strconv.Atoi(_p)
				row = append(row, p)
			}
			updates = append(updates, row)
		}
	}
	return rules, updates
}
