package day10

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func RunP1() {
	f := utils.OpenFile(10, false)
	m, sPts := GetGame(f)
	sum := 0
	for _, s := range sPts {
		sc := GetScore(m, s)
		fmt.Printf("Pt: %d, Score: %d\n", s, sc)
		sum += sc
	}
	fmt.Printf("Score: %d\n", sum)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coord struct {
	x int
	y int
}

func GetScore(maze []string, startPoint Coord) int {
	queue := []Coord{startPoint}
	visited := make(map[Coord]struct{})
	visited[startPoint] = struct{}{}
	score := 0
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		if maze[curr.y][curr.x] == '9' {
			score++
		}

		queue = append(queue, GetNextPoints(maze, visited, curr)...)
	}
	return score
}

func GetGame(f *os.File) ([]string, []Coord) {
	defer f.Close()
	var maze []string
	var startPoints []Coord
	scanner := bufio.NewScanner(f)
	lNum := 0
	for scanner.Scan() {
		line := scanner.Text()
		for i, c := range line {
			if c == '0' {
				startPoints = append(startPoints, Coord{i, lNum})
			}
		}
		maze = append(maze, line)
		lNum++
	}
	return maze, startPoints
}

func GetNextPoints(maze []string, visited map[Coord]struct{}, current Coord) []Coord {
	var out []Coord

	deltas := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	for _, d := range deltas {
		newPoint := Coord{
			current.x + d[0],
			current.y + d[1],
		}
		if _, exists := visited[newPoint]; !exists {
			if IsValidCoord(maze, newPoint) && HeightDiffValid(maze, current, newPoint) {
				visited[newPoint] = struct{}{}
				out = append(out, newPoint)
			}
		}
	}

	return out
}

func HeightDiffValid(maze []string, a, b Coord) bool {
	_l, _r := maze[a.y][a.x], maze[b.y][b.x]
	if _r == '.' {
		return false
	}
	l, _ := strconv.Atoi(string(_l))
	r, _ := strconv.Atoi(string(_r))
	return l < r && r-l == 1
}

func IsValidCoord(maze []string, coord Coord) bool {
	Y, X := len(maze), len(maze[0])
	return coord.x >= 0 && coord.x < X && coord.y >= 0 && coord.y < Y
}
