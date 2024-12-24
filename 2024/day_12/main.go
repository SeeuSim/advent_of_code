package day12

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
)

const (
	topLeft     = 0
	topRight    = 1
	bottomLeft  = 2
	bottomRight = 3
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
	f := utils.OpenFile(12, false)
	_, regions := GetGame(f)
	s := 0
	for _, r := range regions {
		s += r.area * r.sides
	}
	fmt.Printf("Sum: %d\n", s)
}

type Coords struct {
	x, y int
}

type Region struct {
	area      int
	perimeter int
	sides     int
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

	region.area++
	region.perimeter += (4 - len(neighbours))
	seen[curr] = struct{}{}
	region.sides += CheckCorners(maze, curr)

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

func CheckCorners(maze []string, curr Coords) int {
	out := 0
	pType := maze[curr.y][curr.x]
	Y, X, x, y := len(maze), len(maze[0]), curr.x, curr.y

	corners := []Coords{
		{0, 0}, {0, Y - 1}, {X - 1, 0}, {X - 1, Y - 1},
	}
	for _, c := range corners {
		if x == c.x && y == c.y {
			out++
			break
		}
	}

	isValid := func(c Coords) bool {
		return c.y >= 0 && c.y < Y && c.x >= 0 && c.x < X
	}
	// Check edge cases for corners
	isLimit := func(crnr int) bool {
		switch crnr {
		case topLeft:
			return (x > 0 && y == 0) || x == 0 && y > 0
		case topRight:
			return (x < X-1 && y == 0) || x == X-1 && y > 0
		case bottomLeft:
			return (x > 0 && y == Y-1) || x == 0 && y < Y-1
		case bottomRight:
			return (x < X-1 && y == Y-1) || x == X-1 && y < Y-1
		}
		return false
	}
	getType := func(c Coords) byte {
		return maze[c.y][c.x]
	}

	outsideCorners := [][]Coords{
		{{-1, 0}, {0, -1}}, // Top Left
		{{1, 0}, {0, -1}},  // Top Right
		{{-1, 0}, {0, 1}},  // Bottom Left
		{{1, 0}, {0, 1}},   // Bottom Right
	}
	for i, oC := range outsideCorners {
		c1, c2 := Coords{x + oC[0].x, y + oC[0].y}, Coords{x + oC[1].x, y + oC[1].y}
		isV1, isV2 := isValid(c1), isValid(c2)
		if (isV1 && isV2 && getType(c1) != pType && getType(c2) != pType) ||
			(isV1 && isLimit(i) && getType(c1) != pType) ||
			(isV2 && isLimit(i) && getType(c2) != pType) {
			out++
		}
	}

	insideCorners := [][]Coords{
		{{1, 0}, {0, 1}},   // Top Left
		{{-1, 0}, {0, 1}},  // Top Right
		{{1, 0}, {0, -1}},  // Bottom Left
		{{-1, 0}, {0, -1}}, // Bottom Right
	}
	for _, iC := range insideCorners {
		c1, c2 := iC[0], iC[1]
		check := Coords{x + c1.x, y + c2.y}
		if !isValid(check) {
			continue
		}
		c1, c2 = Coords{x + c1.x, y}, Coords{x, y + c2.y}
		if getType(c1) == pType && getType(c2) == pType && getType(check) != pType {
			out++
		}
	}
	return out
}

func IsDiffOutsideCorner(maze []string, coord, delta Coords) bool {
	newC := Coords{coord.x + delta.x, coord.y + delta.y}
	nX, nY := newC.x, newC.y
	if nX < 0 || nX >= len(maze[0]) || nY < 0 || nY >= len(maze) {
		return false
	}
	return maze[coord.y][coord.x] != maze[newC.y][newC.x]
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
