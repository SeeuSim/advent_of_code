package day20

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
)

func RunP1() {
	f := utils.OpenFile(20, false)
	maze, s, e := GetGame(f)
	X, Y := len(maze[0]), len(maze)

	deltas := []Coord{
		{0, -1}, {1, 0}, {0, 1}, {-1, 0},
	}

	queue := []QueueNode{{s, 0, make(map[Coord]int)}}

	var visited map[Coord]int
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]
		coord, time, seen := curr.coord, curr.time, curr.seen
		seen[coord] = time

		if coord == e {
			visited = CopyMap(seen)
			fmt.Println("End Reached", len(visited))
			break
		}

		for _, d := range deltas {
			nC := Coord{coord.x + d.x, coord.y + d.y}
			_, s := seen[nC]
			if nC.x >= 0 && nC.x < X && nC.y >= 0 && nC.y < Y && !s && maze[nC.y][nC.x] != '#' {
				queue = append(queue, QueueNode{nC, time + 1, CopyMap(seen)})
			}
		}
	}

	deltas2 := []Coord{
		{2, 0}, {-2, 0}, {0, 2}, {0, -2},
	}

	c := 0
	for k, t := range visited {
		for _, d := range deltas2 {
			if v, s := visited[Coord{k.x + d.x, k.y + d.y}]; s && (v-t) >= 102 {
				c++
			}
		}
	}
	fmt.Println("Num Cheats:", c)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coord struct {
	x, y int
}

type QueueNode struct {
	coord Coord
	time  int
	seen  map[Coord]int
}

func CopyMap(seen map[Coord]int) map[Coord]int {
	out := make(map[Coord]int)
	for k, v := range seen {
		out[k] = v
	}
	return out
}

func GetGame(f *os.File) ([][]byte, Coord, Coord) {
	defer f.Close()
	var out [][]byte
	scanner := bufio.NewScanner(f)
	var s, e Coord
	for scanner.Scan() {
		line := scanner.Text()
		var row []byte
		for cNum, c := range []byte(line) {
			if c == 'S' {
				s = Coord{cNum, len(out)}
			} else if c == 'E' {
				e = Coord{cNum, len(out)}
			}
			row = append(row, c)
		}
		out = append(out, row)
	}

	return out, s, e
}
