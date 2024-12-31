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
	_, path := GetPathCost(maze, start, end)
	c := GetSafeTiles(maze, path, start, end)
	fmt.Printf("Safe Tiles: %d\n", c)

}

func GetPathCost(maze []string, start, end Coord) (int, map[Coord]int) {
	seen := make(map[DirNode]struct{})

	path := make(map[Coord]int)
	queue := pq.NewWith(Cmp)
	queue.Enqueue(PQNode{DirNode{start, east}, 0, path})

	for !queue.Empty() {
		curr, _ := queue.Dequeue()
		currNode := curr.(PQNode)
		dirNode := currNode.dirNode
		coord, dir := dirNode.coord, dirNode.dir

		if _, s := seen[dirNode]; s {
			continue
		}
		currNode.path[coord] = currNode.cost
		if coord == end {
			return currNode.cost, currNode.path
		}
		seen[dirNode] = struct{}{}

		for _, candidate := range GetNeighbours(maze, coord, dir, currNode.cost, currNode.path) {
			if _, exists := seen[candidate.dirNode]; !exists {
				queue.Enqueue(candidate)
			}
		}
	}
	return -1, make(map[Coord]int)
}

func GetSafeTiles(maze []string, path map[Coord]int, start, end Coord) int {
	queue := pq.NewWith(Cmp)
	queue.Enqueue(PQNode{DirNode{start, east}, 0, make(map[Coord]int)})
	seen := make(map[DirNode]struct{})
	newCoords := make(map[Coord]struct{})

	for !queue.Empty() {
		e, _ := queue.Dequeue()
		curr := e.(PQNode)
		dirNode := curr.dirNode
		coord, dir := dirNode.coord, dirNode.dir

		// Expand curr and see if path leads to nodes not on the best path
		if cost, exists := path[coord]; exists && cost == curr.cost {
			for pt := range curr.path {
				if _, exists := path[pt]; !exists {
					newCoords[pt] = struct{}{}
				}
			}
		}

		if _, exists := seen[dirNode]; exists {
			continue
		}
		curr.path[coord] = curr.cost
		seen[dirNode] = struct{}{}
		if coord == end {
			continue
		}

		for _, candidate := range GetNeighbours(maze, coord, dir, curr.cost, curr.path) {
			if _, exists := seen[candidate.dirNode]; !exists {
				queue.Enqueue(candidate)
			}
		}
	}

	return len(path) + len(newCoords)
}

func GetNeighbours(maze []string, coord Coord, dir, cost int, path map[Coord]int) []PQNode {
	X, Y := len(maze[0]), len(maze)
	deltas := []Coord{
		{0, -1}, // North
		{1, 0},  // East
		{0, 1},  // South
		{-1, 0}, // West
	}

	var neighbours []PQNode

	fwdCoord := Coord{coord.x + deltas[dir].x, coord.y + deltas[dir].y}
	if fwdCoord.x >= 0 && fwdCoord.x < X && fwdCoord.y >= 0 && fwdCoord.y < Y && maze[fwdCoord.y][fwdCoord.x] != '#' {
		neighbours = append(neighbours, PQNode{DirNode{fwdCoord, dir}, cost + 1, CopyMap(path)})
	}

	switch dir {
	case north, south:
		neighbours = append(
			neighbours,
			PQNode{DirNode{coord, east}, cost + 1000, CopyMap(path)},
			PQNode{DirNode{coord, west}, cost + 1000, CopyMap(path)},
		)
	case east, west:
		neighbours = append(
			neighbours,
			PQNode{DirNode{coord, north}, cost + 1000, CopyMap(path)},
			PQNode{DirNode{coord, south}, cost + 1000, CopyMap(path)},
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
	path    map[Coord]int
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
