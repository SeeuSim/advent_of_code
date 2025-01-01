package day18

import (
	"2024/utils"
	"bufio"
	"fmt"
	"strconv"
	"strings"

	pq "github.com/emirpasic/gods/queues/priorityqueue"
)

const (
	isTest = false
)

type Coord struct {
	x, y int
}

type PQNode struct {
	coord Coord
	cost  int
}

func RunP1() {
	maze, _ := GetGame()
	X, Y := len(maze[0]), len(maze)
	start, end := Coord{0, 0}, Coord{X - 1, Y - 1}

	queue := pq.NewWith(Cmp)
	queue.Enqueue(PQNode{start, 0})

	deltas := []Coord{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	seen := make(map[Coord]struct{})

	for !queue.Empty() {
		e, _ := queue.Dequeue()
		curr := e.(PQNode)
		coord, cost := curr.coord, curr.cost

		if _, e := seen[coord]; e {
			continue
		}

		if coord == end {
			fmt.Println("Cost:", cost)
			break
		}

		seen[coord] = struct{}{}

		for _, d := range deltas {
			nCoord := Coord{coord.x + d.x, coord.y + d.y}
			_, visited := seen[nCoord]
			if nCoord.x < 0 || nCoord.x >= X || nCoord.y < 0 || nCoord.y >= Y || maze[nCoord.y][nCoord.x] == '#' || visited {
				continue
			}
			queue.Enqueue(PQNode{nCoord, cost + 1})
		}
	}
}

func RunP2() {
	maze, coords := GetGame()
	X, Y := len(maze[0]), len(maze)
	sByte := 1024
	if isTest {
		sByte = 12
	}
	for i := sByte; i < len(coords); i++ {
		currCost := -1
		maze[coords[i].y][coords[i].x] = '#'

		start, end := Coord{0, 0}, Coord{X - 1, Y - 1}
		queue := pq.NewWith(Cmp)
		queue.Enqueue(PQNode{start, 0})

		deltas := []Coord{
			{0, -1},
			{1, 0},
			{0, 1},
			{-1, 0},
		}

		seen := make(map[Coord]struct{})

		for !queue.Empty() {
			e, _ := queue.Dequeue()
			curr := e.(PQNode)
			coord, cost := curr.coord, curr.cost

			if _, e := seen[coord]; e {
				continue
			}

			if coord == end {
				currCost = cost
				break
			}

			seen[coord] = struct{}{}

			for _, d := range deltas {
				nCoord := Coord{coord.x + d.x, coord.y + d.y}
				_, visited := seen[nCoord]
				if nCoord.x < 0 || nCoord.x >= X || nCoord.y < 0 || nCoord.y >= Y || maze[nCoord.y][nCoord.x] == '#' || visited {
					continue
				}
				queue.Enqueue(PQNode{nCoord, cost + 1})
			}
		}

		if currCost < 0 {
			fmt.Println(coords[i])
			break
		}
	}
}

func Cmp(a, b interface{}) int {
	return a.(PQNode).cost - b.(PQNode).cost
}

func GetGame() ([][]byte, []Coord) {
	f := utils.OpenFile(18, isTest)
	defer f.Close()
	dim, nBytes := 70, 1024
	if isTest {
		dim = 6
		nBytes = 12
	}
	var out [][]byte
	for y := 0; y <= dim; y++ {
		var row []byte
		for x := 0; x <= dim; x++ {
			row = append(row, '.')
		}
		out = append(out, row)
	}

	var coords []Coord
	nB := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		p := strings.Split(line, ",")
		x, _ := strconv.Atoi(p[0])
		y, _ := strconv.Atoi(p[1])
		nB++
		if nB <= nBytes {
			out[y][x] = '#'
		}
		coords = append(coords, Coord{x, y})
	}

	return out, coords
}
