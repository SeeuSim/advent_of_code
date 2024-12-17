package day06

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"regexp"
)

const (
	up    = 0
	right = 1
	down  = 2
	left  = 3
)

type Coords struct {
	x int
	y int
}

type VectCoord struct {
	x   int
	y   int
	dir int
}

func RunP1() {
	f := utils.OpenFile(6, false)
	defer f.Close()

	maze, s_pos := GetMaze(f)
	nUniquePoints, _ := GetGuardPath(maze, s_pos)

	fmt.Printf("Unique Points Visited: %d\n", nUniquePoints)
}

func RunP2() {
	f := utils.OpenFile(6, false)
	defer f.Close()

	maze, s_pos := GetMaze(f)
	_, path := GetGuardPath(maze, s_pos)

	for point := range path {
		y, x := point.y, point.x
		maze[y] = fmt.Sprintf("%s#%s", maze[y][:x], maze[y][x+1:])

		// Todo: Add impl for Part 2

		maze[y] = fmt.Sprintf("%s.%s", maze[y][:x], maze[y][x+1:])
	}
}

func GetMaze(f *os.File) ([]string, VectCoord) {
	var out []string
	var s_pos Coords
	scanner := bufio.NewScanner(f)
	s_regex := regexp.MustCompile(`\^`)
	l_idx := 0
	for scanner.Scan() {
		line := scanner.Text()
		out = append(out, line)
		match := s_regex.FindStringIndex(line)
		if match != nil {
			s_pos.x = match[0]
			s_pos.y = l_idx
		}
		l_idx += 1
	}
	return out, VectCoord{
		x:   s_pos.x,
		y:   s_pos.y,
		dir: up,
	}
}

func GetGuardPath(maze []string, start_pos VectCoord) (int, map[Coords]struct{}) {
	uniquePoints := 0
	out := make(map[Coords]struct{})
	curr := start_pos
	for {
		if _, exists := out[Coords{curr.x, curr.y}]; !exists {
			out[Coords{curr.x, curr.y}] = struct{}{}
			uniquePoints += 1
		}
		isValid, next := GetNextPos(maze, curr)
		if !isValid {
			break
		}
		curr = next
	}
	return uniquePoints, out
}

func GetNextPos(maze []string, pos VectCoord) (bool, VectCoord) {
	isValid := true
	deltas := []Coords{
		{
			x: 0,
			y: -1,
		}, // Up
		{
			x: 1,
			y: 0,
		}, // Right
		{
			x: 0,
			y: 1,
		}, // Down
		{
			x: -1,
			y: 0,
		}, // Left
	}
	out := VectCoord{
		x:   pos.x + deltas[pos.dir].x,
		y:   pos.y + deltas[pos.dir].y,
		dir: pos.dir,
	}

	Y, X := len(maze), len(maze[0])
	isValid = out.x >= 0 && out.x < X && out.y >= 0 && out.y < Y
	if isValid && maze[out.y][out.x] == '#' {
		return GetNextPos(maze, VectCoord{
			pos.x,
			pos.y,
			TurnRight(pos.dir),
		})
	}
	return isValid, out
}

func TurnRight(dir int) int {
	return (dir + 1) % 4
}
