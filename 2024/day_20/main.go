package day20

import (
	"2024/utils"
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
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

	sortedPath := make(PairList, len(visited))
	i := 0
	for k, v := range visited {
		sortedPath[i] = Pair{k, v}
		i++
	}
	sort.Sort(sortedPath)

	c := 0
	for t2 := 100; t2 < len(visited); t2++ {
		for t1 := 0; t1 < t2-100; t1++ {
			p1 := sortedPath[t1]
			p2 := sortedPath[t2]
			dist := int(math.Abs(float64(p1.coord.x-p2.coord.x))) + int(math.Abs(float64(p1.coord.y-p2.coord.y)))
			if dist <= 20 && t2-t1-dist >= 100 {
				c++
			}
		}
	}
	fmt.Println("Num Cheats:", c)
}

type Coord struct {
	x, y int
}

type QueueNode struct {
	coord Coord
	time  int
	seen  map[Coord]int
}

type Pair struct {
	coord Coord
	time  int
}

type PairList []Pair

func (p PairList) Len() int {
	return len(p)
}

func (p PairList) Less(i, j int) bool {
	return p[i].time < p[j].time
}

func (p PairList) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
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
