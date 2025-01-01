package day17

import (
	"2024/utils"
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(17, false)
	g := GetGame(f)
	instructions := g.instructions
	currInst := 0
	output := []string{}
	for currInst < len(instructions) {
		curr := instructions[currInst]
		switch curr.opcode {
		case adv:
			g.a = g.a / int(math.Pow(float64(2), float64(GetComboOperand(curr.operand, g))))
		case bxl:
			g.b = g.b ^ curr.operand
		case bst:
			g.b = GetComboOperand(curr.operand, g) % 8
		case jnz:
			if g.a != 0 {
				currInst = curr.operand / 2
				continue
			}
		case bxc:
			g.b = g.b ^ g.c
		case out:
			output = append(output, strconv.Itoa(GetComboOperand(curr.operand, g)%8))
		case bdv:
			g.b = g.a / int(math.Pow(float64(2), float64(GetComboOperand(curr.operand, g))))
		case cdv:
			g.c = g.a / int(math.Pow(float64(2), float64(GetComboOperand(curr.operand, g))))
		}
		currInst++
	}
	fmt.Println(strings.Join(output, ","))
}

func RunP2() {
	// TODO: Implement Part 2
}

type Game struct {
	a, b, c      int
	instructions []Instruction
}

type Instruction struct {
	opcode, operand int
}

const (
	adv = 0
	bxl = 1
	bst = 2
	jnz = 3
	bxc = 4
	out = 5
	bdv = 6
	cdv = 7
)

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
			var inst Instruction
			for i, p := range parts {
				p, _ := strconv.Atoi(p)
				if i%2 == 0 {
					inst.opcode = p
				} else {
					inst.operand = p
					out.instructions = append(out.instructions, inst)
					inst = Instruction{}
				}
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
