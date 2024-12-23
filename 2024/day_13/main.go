package day13

import (
	"2024/utils"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(13, false)
	c := 0
	counts := GetGame(f)
	for _, _c := range counts {
		c += _c.cost
	}
	fmt.Printf("Cost: %d\n", c)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Entry struct {
	cost int
}

type Coord struct {
	x int
	y int
}

func GetGame(f *os.File) []Entry {
	defer f.Close()
	var out []Entry
	_content, _ := os.ReadFile(f.Name())
	content := string(_content)
	parts := strings.Split(content, "\n\n")
	for _, p := range parts {
		out = append(out, FromString(p))
	}
	return out
}

func FromString(content string) Entry {
	var out Entry
	parts := strings.Split(content, "\n")
	A, B, target := GetCoords(parts[0], false), GetCoords(parts[1], false), GetCoords(parts[2], true)
	det := A.x*B.y - A.y*B.x

	a := (target.x*B.y - target.y*B.x)
	b := (target.y*A.x - target.x*A.y)

	if a%det == 0 && b%det == 0 {
		out.cost += (a*3/det + b/det)
	}
	return out
}

func GetCoords(s string, isPrize bool) Coord {
	var reg *regexp.Regexp
	if isPrize {
		reg = regexp.MustCompile(`=(\d+)`)
	} else {
		reg = regexp.MustCompile(`\+(\d+)`)
	}
	coords := reg.FindAllStringSubmatch(s, -1)
	X, _ := strconv.Atoi(coords[0][1])
	Y, _ := strconv.Atoi(coords[1][1])
	return Coord{X, Y}
}

/*
	Minimise number of button A, maximise number of button B

	Bottom up DP:

	{0,0}: 0
	{}
*/
