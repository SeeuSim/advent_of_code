package day17

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
	f := utils.OpenFile(17, false)
	g := GetGame(f)
	a, b, c := g.a, 0, 0
	outputS := []string{}
	iPtr := 0
	for iPtr < len(g.instructions) {
		comboOp := GetComboOperand(g.instructions[iPtr+1], Game{a: a, b: b, c: c})

		opCode := g.instructions[iPtr]
		if opCode == 0 {
			a >>= comboOp
		} else if opCode == 1 {
			b ^= g.instructions[iPtr+1]
		} else if opCode == 2 {
			b = comboOp & 7
		} else if opCode == 3 {
			if a != 0 {
				iPtr = g.instructions[iPtr+1] - 2
			}
		} else if opCode == 4 {
			b ^= c
		} else if opCode == 5 {
			outputS = append(outputS, strconv.Itoa(comboOp%8))
		} else if opCode == 6 {
			b = a >> comboOp
		} else if opCode == 7 {
			c = a >> comboOp
		}
		iPtr += 2
	}
	fmt.Println(strings.Join(outputS, ","))
}

func RunP2() {
	f := utils.OpenFile(17, false)
	game := GetGame(f)

	instructions := game.instructions

	Solve(len(instructions)-1, 0, instructions)
}

func Solve(idx, A int, game []int) bool {
	if idx < 0 {
		fmt.Println(A)
		return true
	}

	for d := 0; d < 8; d++ {
		a, b, c := (A<<3)|d, 0, 0
		iPtr := 0
		output := 0
		for iPtr < len(game) {
			comboOp := GetComboOperand(game[iPtr+1], Game{a: a, b: b, c: c})

			opCode := game[iPtr]
			if opCode == 0 {
				a >>= comboOp
			} else if opCode == 1 {
				b ^= game[iPtr+1]
			} else if opCode == 2 {
				b = comboOp & 7
			} else if opCode == 3 {
				if a != 0 {
					iPtr = game[iPtr+1] - 2
				}
			} else if opCode == 4 {
				b ^= c
			} else if opCode == 5 {
				output = comboOp % 8
				break
			} else if opCode == 6 {
				b = a >> comboOp
			} else if opCode == 7 {
				c = a >> comboOp
			}
			iPtr += 2
		}

		if output == game[idx] && Solve(idx-1, (A<<3)|d, game) {
			return true
		}
	}

	return false
}

type Game struct {
	a, b, c      int
	instructions []int
}

func GetComboOperand(operand int, g Game) int {
	switch operand {
	case 4:
		return g.a
	case 5:
		return g.b
	case 6:
		return g.c
	case 0, 1, 2, 3:
		return operand
	}
	return -1
}

func GetGame(f *os.File) Game {
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out Game
	l := -1
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		l++
		if l >= 3 {
			parts := strings.Split(line[9:], ",")
			for _, p := range parts {
				p, _ := strconv.Atoi(p)
				out.instructions = append(out.instructions, p)
			}
		}
		dReg := regexp.MustCompile(`\d+`)
		nPort := dReg.FindStringSubmatch(line)[0]
		num, _ := strconv.Atoi(nPort)
		switch l {
		case 0:
			out.a = num
		case 1:
			out.b = num
		case 2:
			out.c = num
		}
	}
	return out
}
