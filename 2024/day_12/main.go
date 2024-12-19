package day12

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
)

func RunP1() {
	f := utils.OpenFile(12, false)
	_, regions := GetGame(f)
	s := 0
	for _, r := range regions {
		s += r.area * r.perimeter
	}
	fmt.Printf("Sum: %d\n", s)
}

func RunP2() {
	// TODO: Implement Part 2
}

type Coords struct {
	x int
	y int
}

type Region struct {
	perimeter int
	area      int
}

func GetGame(f *os.File) ([]string, []Region) {
	defer f.Close()

	var maze []string
	var regions []Region
	seen := make(map[Coords]struct{})
	scanner := bufio.NewScanner(f)
	lNum := 0
	for scanner.Scan() {
		line := scanner.Text()
		maze = append(maze, line)
		lNum++
	}
	Y, X := len(maze), len(maze[0])
	for y := 0; y < Y; y++ {
		for x := 0; x < X; x++ {
			if _, exists := seen[Coords{x, y}]; exists {
				continue
			}
			regions = append(regions, GetRegion(maze, seen, Region{}, Coords{x, y}))
		}
	}
	return maze, regions
}

func GetRegion(maze []string, seen map[Coords]struct{}, region Region, curr Coords) Region {
	if _, exists := seen[curr]; exists {
		return region
	}

	// Even neighbours which are visited already will be generated.
	// This gives the accurate number of neighbours for the current tile
	neighbours := GetNeighbours(maze, curr)

	region.area += 1
	region.perimeter += (4 - len(neighbours))
	seen[curr] = struct{}{}

	for _, c := range neighbours {
		region = GetRegion(maze, seen, region, c)
	}

	return region
}

func GetNeighbours(maze []string, curr Coords) []Coords {
	var out []Coords
	deltas := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for _, d := range deltas {
		newCoord := Coords{curr.x + d[0], curr.y + d[1]}
		if IsValidNeighbour(maze, curr, newCoord) {
			out = append(out, newCoord)
		}
	}
	return out
}

func IsValidNeighbour(maze []string, coord, candidate Coords) bool {
	Y, X := len(maze), len(maze[0])
	if candidate.y < 0 || candidate.y >= Y || candidate.x < 0 || candidate.x >= X {
		return false
	}
	return maze[candidate.y][candidate.x] == maze[coord.y][coord.x]
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
