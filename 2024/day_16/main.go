package day16

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"

	pq "github.com/emirpasic/gods/queues/priorityqueue"
)

func RunP1() {
	f := utils.OpenFile(16, false)
	maze, start, end := GetGame(f)
	cost, _ := GetPathCost(maze, start, end)
	fmt.Printf("Cost: %d\n", cost)
}

func RunP2() {
	f := utils.OpenFile(16, false)
	maze, start, end := GetGame(f)
	cost, distFromStart := GetPathCost(maze, start, end)
	_, distFromEnd := GetBackwardsPathCost(maze, start, end)

	s := make(map[Coord]struct{})
	for y := 0; y < len(maze); y++ {
		for x := 0; x < len(maze[0]); x++ {
			for _, dir := range []int{north, south, east, west} {
				idNode := DirNode{Coord{x, y}, dir}
				s1, seen1 := distFromStart[idNode]
				s2, seen2 := distFromEnd[idNode]
				if seen1 && seen2 && (s1+s2 == cost) {
					s[Coord{x, y}] = struct{}{}
				}
			}
		}
	}
	fmt.Printf("%d\n", len(s))
}

func GetPathCost(maze []string, start, end Coord) (int, map[DirNode]int) {
	seen := make(map[DirNode]int)
	out := -1
	queue := pq.NewWith(Cmp)
	queue.Enqueue(PQNode{DirNode{start, east}, 0})

	for !queue.Empty() {
		curr, _ := queue.Dequeue()
		currNode := curr.(PQNode)
		dirNode := currNode.dirNode
		coord, dir := dirNode.coord, dirNode.dir

		if _, s := seen[dirNode]; s {
			continue
		}
		if coord == end && out < 0 {
			out = currNode.cost
		}
		seen[dirNode] = currNode.cost

		for _, candidate := range GetNeighbours(maze, coord, dir, currNode.cost, false) {
			if _, exists := seen[candidate.dirNode]; !exists {
				queue.Enqueue(candidate)
			}
		}
	}
	return out, seen
}

func GetBackwardsPathCost(maze []string, start, end Coord) (int, map[DirNode]int) {
	seen := make(map[DirNode]int)
	out := -1

	queue := pq.NewWith(Cmp)
	for _, dir := range []int{north, south, east, west} {
		queue.Enqueue(PQNode{DirNode{end, dir}, 0})
	}

	for !queue.Empty() {
		curr, _ := queue.Dequeue()
		currNode := curr.(PQNode)
		dirNode := currNode.dirNode
		coord, dir := dirNode.coord, dirNode.dir

		if _, s := seen[dirNode]; s {
			continue
		}
		if coord == start && out < 0 {
			out = currNode.cost
		}
		seen[dirNode] = currNode.cost

		for _, candidate := range GetNeighbours(maze, coord, dir, currNode.cost, true) {
			if _, exists := seen[candidate.dirNode]; !exists {
				queue.Enqueue(candidate)
			}
		}
	}
	return out, seen
}

func GetNeighbours(maze []string, coord Coord, dir, cost int, isBackwards bool) []PQNode {
	X, Y := len(maze[0]), len(maze)
	deltas := []Coord{
		{0, -1}, // North
		{1, 0},  // East
		{0, 1},  // South
		{-1, 0}, // West
	}

	var neighbours []PQNode

	d := dir
	if isBackwards {
		d = (d + 2) % 4
	}

	fwdCoord := Coord{coord.x + deltas[d].x, coord.y + deltas[d].y}
	if fwdCoord.x >= 0 && fwdCoord.x < X && fwdCoord.y >= 0 && fwdCoord.y < Y && maze[fwdCoord.y][fwdCoord.x] != '#' {
		neighbours = append(neighbours, PQNode{DirNode{fwdCoord, dir}, cost + 1})
	}

	switch dir {
	case north, south:
		neighbours = append(
			neighbours,
			PQNode{DirNode{coord, east}, cost + 1000},
			PQNode{DirNode{coord, west}, cost + 1000},
		)
	case east, west:
		neighbours = append(
			neighbours,
			PQNode{DirNode{coord, north}, cost + 1000},
			PQNode{DirNode{coord, south}, cost + 1000},
		)
	}
	return neighbours
}

const (
	north = 0
	east  = 1
	south = 2
	west  = 3
)

type Coord struct {
	x, y int
}

type DirNode struct {
	coord Coord
	dir   int
}

type PQNode struct {
	dirNode DirNode
	cost    int
}

func Cmp(a, b interface{}) int {
	cA := a.(PQNode).cost
	cB := b.(PQNode).cost
	return cA - cB // Increasing order
}

func CopyMap(visited map[Coord]int) map[Coord]int {
	new := make(map[Coord]int)
	for k, v := range visited {
		new[k] = v
	}
	return new
}

func GetGame(f *os.File) ([]string, Coord, Coord) {
	var sCoord, eCoord Coord

	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out []string
	for scanner.Scan() {
		line := scanner.Text()
		for col, ch := range line {
			if ch == 'S' {
				sCoord = Coord{col, len(out)}
			} else if ch == 'E' {
				eCoord = Coord{col, len(out)}
			}
		}
		out = append(out, line)
	}
	return out, sCoord, eCoord
}
