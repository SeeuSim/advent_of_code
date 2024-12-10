package day03

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func RunP1() {
	sum := GetStringSum(string(OpenFile()))
	fmt.Printf("Sum: %d\n", sum)
}

func RunP2() {
	dontRegex := regexp.MustCompile(`don't\(\)`)
	doRegex := regexp.MustCompile(`do\(\)`)
	enabled := true
	mem := string(OpenFile())
	sum := 0
	for len(mem) > 0 {
		if enabled {
			dontPos := dontRegex.FindStringIndex(mem)
			if dontPos == nil {
				sum += GetStringSum(mem)
				break
			}
			sum += GetStringSum(mem[:dontPos[0]])
			mem = mem[dontPos[1]:]
			enabled = false
		} else {
			doPos := doRegex.FindStringIndex(mem)
			if doPos == nil {
				break
			}
			mem = mem[doPos[0]:]
			enabled = true
		}
	}
	fmt.Printf("Sum: %d\n", sum)
}

func GetStringSum(mulString string) int {
	sum := 0
	mulRegex := regexp.MustCompile(`mul\(\d+,\d+\)`)
	matches := mulRegex.FindAllString(mulString, -1)
	for i := range matches {
		sum += ProcessMul(matches[i])
	}
	return sum
}

func ProcessMul(command string) int {
	r := regexp.MustCompile(`\d+`)
	matches := r.FindAllString(command, -1)
	lN := matches[0]
	rN := matches[1]
	lV, _ := strconv.Atoi(lN)
	rV, _ := strconv.Atoi(rN)
	return lV * rV
}

func OpenFile() []byte {
	f, e := os.ReadFile("./day_03/input.in")
	if e != nil {
		log.Fatalf("Error reading file: %v", e)
		os.Exit(1)
		return nil
	}
	return f
}
