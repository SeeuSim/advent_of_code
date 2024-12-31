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
	cost := GetPathCost(maze, start, end)
	fmt.Printf("Cost: %d\n", cost)
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetPathCost(maze []string, start, end Coord) int {
	seen := make(map[DirNode]struct{})

	X, Y := len(maze[0]), len(maze)

	deltas := []Coord{
		{0, -1}, // North
		{1, 0},  // East
		{0, 1},  // South
		{-1, 0}, // West
	}

	queue := pq.NewWith(cmp)
	queue.Enqueue(PQNode{DirNode{start, east}, 0})

	for !queue.Empty() {
		curr, _ := queue.Dequeue()
		currNode := curr.(PQNode)
		dirNode := currNode.dirNode
		coord, dir := dirNode.coord, dirNode.dir

		if _, s := seen[dirNode]; s {
			continue
		}
		if coord == end {
			return currNode.cost
		}
		seen[dirNode] = struct{}{}

		neighbours := []PQNode{}

		fwdCoord := Coord{coord.x + deltas[dir].x, coord.y + deltas[dir].y}
		if fwdCoord.x >= 0 && fwdCoord.x < X && fwdCoord.y >= 0 && fwdCoord.y < Y && maze[fwdCoord.y][fwdCoord.x] != '#' {
			neighbours = append(neighbours, PQNode{DirNode{fwdCoord, dir}, currNode.cost + 1})
		}

		switch dir {
		case north, south:
			neighbours = append(
				neighbours,
				PQNode{DirNode{coord, east}, currNode.cost + 1000},
				PQNode{DirNode{coord, west}, currNode.cost + 1000},
			)
		case east, west:
			neighbours = append(
				neighbours,
				PQNode{DirNode{coord, north}, currNode.cost + 1000},
				PQNode{DirNode{coord, south}, currNode.cost + 1000},
			)
		}
		for _, candidate := range neighbours {
			if _, exists := seen[candidate.dirNode]; !exists {
				queue.Enqueue(candidate)
			}
		}
	}
	return -1
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

func cmp(a, b interface{}) int {
	cA := a.(PQNode).cost
	cB := b.(PQNode).cost
	return cA - cB // Increasing order
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
