package day07

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
	f := utils.OpenFile(7, false)
	equations := GetGame(f)
	out := 0
	for _, eqn := range equations {
		if eqn.IsValidP1() {
			out += eqn.target
		}
	}
	fmt.Printf("Sum: %d\n", out)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Equation struct {
	target   int
	operands []int
}

const (
	plus  = '+'
	times = '*'
)

func (e *Equation) ToString() string {
	return fmt.Sprintf("Target: %d, Ops: %d", e.target, e.operands)
}

func (e *Equation) IsValidP1() bool {
	return IsTarget(e.target, e.operands[0], e.operands[1:])
}

func Calc(a, b int, op byte) int {
	switch op {
	case plus:
		return a + b
	case times:
		return a * b
	}
	return 0
}

func IsTarget(target, curr int, ops []int) bool {
	if len(ops) == 0 {
		return target == curr
	}
	if curr > target {
		return false
	}
	return IsTarget(target, Calc(curr, ops[0], plus), ops[1:]) || IsTarget(target, Calc(curr, ops[0], times), ops[1:])
}

func GetGame(f *os.File) []Equation {
	scanner := bufio.NewScanner(f)
	var out []Equation
	for scanner.Scan() {
		line := scanner.Text()
		out = append(out, FromString(line))
	}
	return out
}

func FromString(input string) Equation {
	targetReg := regexp.MustCompile(`(\d+):`)
	opsReg := regexp.MustCompile(`:\s+(.+)`)
	groups := targetReg.FindAllStringSubmatch(input, -1)

	_target := groups[0][1]
	target, _ := strconv.Atoi(_target)

	ops := opsReg.FindAllStringSubmatch(input, -1)[0][1]
	var opSlice []int
	for _, o := range strings.Fields(ops) {
		op, _ := strconv.Atoi(o)
		opSlice = append(opSlice, op)
	}

	return Equation{
		target,
		opSlice,
	}
}
