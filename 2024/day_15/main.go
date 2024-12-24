package day15

import (
	"2024/utils"
	"fmt"
	"os"
	"strings"
)

const (
	UP    = '^'
	DOWN  = 'v'
	LEFT  = '<'
	RIGHT = '>'
)

const (
	EMPTY byte = '.'
	BOX   byte = 'O'
	WALL  byte = '#'
	ROBOT byte = '@'
)

func RunP1() {
	f := utils.OpenFile(15, false)
	maze, moves, curr := GetGame(f)

	deltas := map[rune]Coord{
		UP:    {0, -1},
		DOWN:  {0, 1},
		LEFT:  {-1, 0},
		RIGHT: {1, 0},
	}

	fmt.Printf("%s\n", maze)
	s := 0
	for _, move := range moves {
		curr = MakeMove(maze, curr, deltas[move])
	}

	for y := 0; y < len(maze); y++ {
		for x := 0; x < len(maze[0]); x++ {
			if maze[y][x] == BOX {
				s += (100*y + x)
			}
		}
	}

	fmt.Printf("%s\n", maze)
	fmt.Printf("Sum: %d\n", s)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coord struct {
	x, y int
}

func GetGame(f *os.File) ([][]byte, string, Coord) {
	defer f.Close()
	var out [][]byte
	_content, _ := os.ReadFile(f.Name())
	content := strings.Split(string(_content), "\n\n")
	maze, moves := content[0], content[1]
	var sPos Coord
	for lNum, line := range strings.Split(maze, "\n") {
		for cNum := range []byte(line) {
			if line[cNum] == ROBOT {
				sPos = Coord{cNum, lNum}
			}
		}

		out = append(out, []byte(line))
	}

	return out, moves, sPos
}

func MakeMove(maze [][]byte, s, delta Coord) Coord {
	base := s
	nextCoord := Coord{s.x + delta.x, s.y + delta.y}
	switch maze[nextCoord.y][nextCoord.x] {
	case EMPTY:
		maze[nextCoord.y][nextCoord.x] = ROBOT
		maze[s.y][s.x] = EMPTY
		return nextCoord
	case BOX:
		curr := Coord{nextCoord.x, nextCoord.y}
		for {
			switch maze[curr.y][curr.x] {
			case WALL:
				return s
			case EMPTY:
				maze[s.y][s.x] = EMPTY
				maze[nextCoord.y][nextCoord.x] = ROBOT
				for curr != nextCoord {
					maze[curr.y][curr.x] = BOX
					curr.y -= delta.y
					curr.x -= delta.x
				}
				return nextCoord
			}
			curr.y += delta.y
			curr.x += delta.x
		}
	case WALL:
		return s
	}

	return base
}
