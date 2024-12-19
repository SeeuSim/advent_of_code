package day12

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
)

func RunP1() {
	f := utils.OpenFile(12, true)
	_, regions := GetGame(f)
	fmt.Printf("%s\n", regions)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coords struct {
	coordType byte
	x         int
	y         int
}

type Region struct {
	regionType byte
	regionId   int
	nodes      []Coords
}

func (c Coords) String() string {
	return fmt.Sprintf("{x:%d,y:%d,c:%c}", c.x, c.y, c.coordType)
}

func (r Region) String() string {
	return fmt.Sprintf("{ID:%d,T:%c,N:%s}", r.regionId, r.regionType, r.nodes)
}

func GetGame(f *os.File) ([]string, []Region) {
	defer f.Close()

	var maze []string
	var regions []Region
	nodeMap := make(map[Coords]struct{})
	scanner := bufio.NewScanner(f)
	lNum := 0
	for scanner.Scan() {
		line := scanner.Text()
		for i, r := range []byte(line) {
			nodeMap[Coords{r, i, lNum}] = struct{}{}
		}
		maze = append(maze, line)
		lNum++
	}
	id := 0
	for len(nodeMap) > 0 {
		var firstNode Coords
		for k := range nodeMap {
			firstNode = k
			break
		}
		regions = append(regions, GetRegion(maze, nodeMap, firstNode, id))
		id++
	}
	return maze, regions
}

func GetRegion(maze []string, nodeMap map[Coords]struct{}, firstNode Coords, id int) Region {
	var out Region
	queue := []Coords{firstNode}
	out.regionId = id
	out.regionType = firstNode.coordType
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]
		if _, exists := nodeMap[curr]; exists {
			delete(nodeMap, curr)
		}
		out.nodes = append(out.nodes, curr)
		queue = append(queue, GetNeighbours(maze, nodeMap, curr)...)
	}
	return out
}

func GetNeighbours(maze []string, nodeMap map[Coords]struct{}, node Coords) []Coords {
	var out []Coords
	deltas := [4][2]int{
		{0, 1},
		{0, -1},
		{1, 0},
		{-1, 0},
	}
	for _, d := range deltas {
		newCoord := Coords{node.coordType, node.x + d[0], node.y + d[1]}
		if _, exists := nodeMap[newCoord]; !exists {
			continue
		}
		if IsValidCoord(maze, newCoord) {
			delete(nodeMap, newCoord)
			out = append(out, newCoord)
		}
	}
	return out
}

func IsValidCoord(maze []string, node Coords) bool {
	Y, X := len(maze), len(maze[0])
	if node.x >= X || node.x < 0 || node.y >= Y || node.y < 0 {
		return false
	}
	if node.coordType != maze[node.y][node.x] {
		return false
	}
	return true
}

/**
Whole region: shape

1. Build a map of positions
2. While map is not empty:
  - Take the first position from the map
  - Explore outwards from this position
  - For each neighbouring node of the same type, add to the region IFF:
    - node is of same type
    - node is still in map.
  - Generate Region
  - Once no more nodes are addable to the Region, return

*/
