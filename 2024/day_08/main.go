package day08

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
)

func RunP1() {
	f := utils.OpenFile(8, false)
	maze, pos := GetMaze(f)
	seen := make(map[Coord]struct{})
	nNodes := 0
	for _, v := range pos {
		for l := range v {
			for r := l + 1; r < len(v); r++ {
				antiNodes := GetAntinodes(maze, v[l], v[r])
				for _, node := range antiNodes {
					if _, exists := seen[node]; !exists {
						seen[node] = struct{}{}
						nNodes += 1
					}
				}
			}
		}
	}
	fmt.Printf("Num Nodes: %d\n", nNodes)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coord struct {
	x int
	y int
}

func (c Coord) String() string {
	return fmt.Sprintf("{%d,%d}", c.y, c.x)
}

func GetMaze(f *os.File) ([][]byte, map[byte][]Coord) {
	var out [][]byte
	pos := make(map[byte][]Coord)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	lNum := 0
	for scanner.Scan() {
		line := scanner.Text()
		var row []byte
		for i, ch := range []byte(line) {
			row = append(row, ch)
			if ch != '.' {
				pos[ch] = append(pos[ch], Coord{i, lNum})
			}
		}
		out = append(out, row)
		lNum += 1
	}

	return out, pos
}

func GetAntinodes(maze [][]byte, a, b Coord) []Coord {
	var out []Coord
	dist := Coord{
		a.x - b.x,
		a.y - b.y,
	}
	left := Coord{
		a.x + dist.x,
		a.y + dist.y,
	}
	if IsValidCoord(maze, left) {
		out = append(out, left)
	}
	right := Coord{
		b.x - dist.x,
		b.y - dist.y,
	}
	if IsValidCoord(maze, right) {
		out = append(out, right)
	}
	return out
}

func IsValidCoord(maze [][]byte, c Coord) bool {
	X, Y := len(maze[0]), len(maze)
	return c.x >= 0 && c.x < X && c.y >= 0 && c.y < Y
}
