package day04

import (
	utils "2024/utils"
	"bufio"
	"fmt"
	"os"
)

func RunP1() {
	file := utils.OpenFile(4, false)
	defer file.Close()

	maze, sPos := GetMaze(file)
	nRows := len(maze)
	nCols := len(maze[0])
	cmpStr := "XMAS"
	nXmas := 0

	posOffsets := [][2]int{
		{0, 1},
		{0, -1},
		{1, 0},
		{-1, 0},
		{-1, -1},
		{1, -1},
		{-1, 1},
		{1, 1},
	}
	for i := range sPos {
		start := sPos[i]
		ro, col := start[0], start[1]

		for offsetI := range posOffsets {
			offset := posOffsets[offsetI]
			isXmas := true
			for i := 1; i < 4; i++ {
				nR, nC := (ro + offset[0]*i), (col + offset[1]*i)
				if nR < 0 || nR >= nRows || nC < 0 || nC >= nCols { // out of bounds
					isXmas = false
					break
				} else if maze[nR][nC] != cmpStr[i] { // not XMAS
					isXmas = false
					break
				}
			}
			if isXmas {
				nXmas++
			}
		}
	}
	fmt.Printf("Num XMAS: %d\n", nXmas)
}

func RunP2() {
	file := utils.OpenFile(4, false)
	defer file.Close()

	maze, _ := GetMaze(file)
	nRows := len(maze)
	nCols := len(maze[0])

	nXmas := 0
	// Rotate through the maze, in 3x3 blocks
	for ro := 0; ro < nRows-2; ro++ {
		// Check if is X-MAS
		for co := 0; co < nCols-2; co++ {
			// Check if window center is 'A'
			if maze[ro+1][co+1] != 'A' {
				continue
			}

			// Check corners - 4 possible configs
			if maze[ro][co] == maze[ro][co+2] && maze[ro+2][co] == maze[ro+2][co+2] && ((maze[ro][co] == 'M' && maze[ro+2][co] == 'S') || (maze[ro][co] == 'S' && maze[ro+2][co] == 'M')) {
				// M both top or both bottom
				nXmas += 1
			} else if maze[ro][co] == maze[ro+2][co] && maze[ro][co+2] == maze[ro+2][co+2] && ((maze[ro][co] == 'M' && maze[ro][co+2] == 'S') || (maze[ro][co] == 'S' && maze[ro][co+2] == 'M')) {
				// M both left or both right
				nXmas += 1
			}

		}
	}
	fmt.Printf("N XMAS: %d\n", nXmas)
}

func GetMaze(f *os.File) ([]string, [][2]int) {
	scanner := bufio.NewScanner(f)
	var out []string
	var pos [][2]int
	rNum := 0
	for scanner.Scan() {
		line := scanner.Text()
		for i := range line {
			curr := line[i]
			if curr == 'X' {
				pos = append(pos, [2]int{rNum, i})
			}
		}
		rNum += 1
		out = append(out, line)
	}
	return out, pos
}
