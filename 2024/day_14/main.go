package day14

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const (
	isTest = false
	nMoves = 100
	Y      = 103
	X      = 101
	tL     = 0
	tR     = 1
	bL     = 2
	bR     = 3
)

func RunP1() {
	f := utils.OpenFile(14, isTest)
	g := GetGame(f)
	var sc [4]int
	for _, rbt := range g {
		p, v := rbt[0], rbt[1]
		p.x = WrappingAdd(p.x, nMoves*v.x, true)
		p.y = WrappingAdd(p.y, nMoves*v.y, false)
		quadrant := AssignQuadrant(p)
		fmt.Printf("%d  %d %d\n", rbt, p, quadrant)
		if quadrant >= 0 {
			sc[quadrant]++
		}
	}

	s := sc[0] * sc[1] * sc[2] * sc[3]
	fmt.Printf("Sum: %d %d\n", sc, s)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coord struct {
	x int
	y int
}

func GetGame(f *os.File) [][2]Coord {
	defer f.Close()
	var out [][2]Coord
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		var row [2]Coord
		line := scanner.Text()
		row[0], row[1] = GetCoord(line, `p=(.+)\s`), GetCoord(line, `v=(.+)`)
		out = append(out, row)
	}
	return out
}

func GetCoord(line, r string) Coord {
	partReg := regexp.MustCompile(r)
	part := partReg.FindAllStringSubmatch(line, 1)[0][1]
	parts := strings.Split(part, ",")
	x, _ := strconv.Atoi(parts[0])
	y, _ := strconv.Atoi(parts[1])

	return Coord{x, y}
}

func AssignQuadrant(c Coord) int {
	hX := X / 2
	hY := Y / 2
	if c.x == hX || c.y == hY {
		return -1
	}
	if c.x < hX && c.y < hY {
		return tL
	}
	if c.x > hX && c.y < hY {
		return tR
	}
	if c.x < hX && c.y > hY {
		return bL
	}
	return bR
}

func WrappingAdd(num, add int, isX bool) int {
	out := num + add
	wrapY, wrapX := Y, X
	if isTest {
		wrapY, wrapX = 7, 11
	}
	wrap := func(num, tgt int) int {
		out := num % tgt
		if out < 0 {
			out += tgt
		}
		return out
	}
	if isX {
		return wrap(out, wrapX)
	}
	return wrap(out, wrapY)
}
