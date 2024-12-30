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
	f := utils.OpenFile(15, false)
	maze, moves, curr := GetGameP2(f)
	s := 0

	for _, move := range moves {
		curr = MakeMoveP2(maze, curr, move)
	}

	for y := 0; y < len(maze); y++ {
		for x := 0; x < len(maze[0]); x++ {
			if maze[y][x] == '[' {
				s += (100*y + x)
			}
		}
	}

	fmt.Printf("Sum: %d\n", s)
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

func GetGameP2(f *os.File) ([][]byte, string, Coord) {
	defer f.Close()
	var out [][]byte
	_content, _ := os.ReadFile(f.Name())
	content := strings.Split(string(_content), "\n\n")
	maze, moves := content[0], content[1]
	var sPos Coord
	for lNum, line := range strings.Split(maze, "\n") {
		var row []byte
		for cNum, ch := range []byte(line) {
			switch ch {
			case WALL: // #
				row = append(row, WALL, WALL) // ##
			case BOX: // O
				row = append(row, '[', ']')
			case EMPTY: // .
				row = append(row, EMPTY, EMPTY)
			case ROBOT:
				row = append(row, ROBOT, EMPTY)
				sPos = Coord{cNum * 2, lNum}
			}
		}
		out = append(out, row)
	}
	return out, moves, sPos
}

func MakeMoveP2(maze [][]byte, s Coord, dir rune) Coord {
	deltas := map[rune]Coord{
		UP:    {0, -1},
		DOWN:  {0, 1},
		LEFT:  {-1, 0},
		RIGHT: {1, 0},
	}
	nCoord := Coord{s.x + deltas[dir].x, s.y + deltas[dir].y}
	switch maze[nCoord.y][nCoord.x] {
	case WALL:
		return s
	case EMPTY:
		maze[nCoord.y][nCoord.x] = ROBOT
		maze[s.y][s.x] = EMPTY
		return nCoord
	case '[', ']':
		if CanMoveBoxes(maze, s, dir) {
			return nCoord
		}
	}
	return s
}

func CanMoveBoxes(maze [][]byte, s Coord, dir rune) bool {
	switch dir {
	case LEFT, RIGHT:
		return MoveBoxesHori(maze, s, dir)
	case UP, DOWN:
		return MoveBoxesVert(maze, s, dir)
	}
	return false
}

func MoveBoxesHori(maze [][]byte, s Coord, dir rune) bool {
	delta := Coord{1, 0}
	if dir == LEFT {
		delta = Coord{-1, 0}
	}
	// s is the robot
	curr := Coord{s.x + delta.x, s.y + delta.y}
	for {
		switch maze[curr.y][curr.x] {
		case WALL:
			return false
		case EMPTY:
			// iterate to shuffle from curr to s
			for curr != s {
				maze[curr.y][curr.x], maze[curr.y-delta.y][curr.x-delta.x] = maze[curr.y-delta.y][curr.x-delta.x], maze[curr.y][curr.x]
				curr.x -= delta.x
				curr.y -= delta.y
			}
			return true
		}
		curr.x += delta.x
		curr.y += delta.y
	}
}

func MoveBoxesVert(maze [][]byte, s Coord, dir rune) bool {
	delta := Coord{0, 1}
	if dir == UP {
		delta = Coord{0, -1}
	}
	// s is the robot
	// draw graph from first box and ensure that in the given direction:
	// - each box ends in a child node which is adjacent to an empty space.
	// - else, we can't move
	curr := Coord{s.x + delta.x, s.y + delta.y}
	queue := []Coord{curr}
	if maze[curr.y][curr.x] == '[' {
		queue = append(queue, Coord{curr.x + 1, curr.y})
	} else {
		queue = append(queue, Coord{curr.x - 1, curr.y})
	}

	seen := make(map[Coord]struct{})
	var coords []Coord

	for len(queue) != 0 {
		curr := queue[0]
		queue = queue[1:]
		if _, exists := seen[curr]; exists {
			continue
		}
		seen[curr] = struct{}{}
		coords = append(coords, curr)
		next := Coord{curr.x + delta.x, curr.y + delta.y}
		switch maze[next.y][next.x] {
		case EMPTY:
			continue
		case WALL:
			return false
		case ']':
			queue = append(queue, Coord{next.x - 1, next.y}, next)
		case '[':
			queue = append(queue, next, Coord{next.x + 1, next.y})
		}
	}

	for i := len(coords) - 1; i >= 0; i-- {
		x, y := coords[i].x+delta.x, coords[i].y+delta.y
		maze[y][x] = maze[coords[i].y][coords[i].x]
		maze[coords[i].y][coords[i].x] = EMPTY
	}
	maze[s.y][s.x] = EMPTY
	maze[s.y+delta.y][s.x+delta.x] = ROBOT

	return true
}
