package day06

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"regexp"
)

func RunP1() {
	f := utils.OpenFile(6, false)
	defer f.Close()
	pos_visited := 0
	seen := make(map[int]bool)
	maze, s_pos := GetMaze(f)
	R, C := len(maze), len(maze[0])
	queue := [][2]int{s_pos}

	dir := 0 // 0 - up, 1 - down, 2 - left, 3 - right
	deltas := [][3]int{
		{-1, 0, 3}, // Up
		{1, 0, 2},  // Down
		{0, -1, 0}, // Left
		{0, 1, 1},  // Right
	}

	for len(queue) > 0 {
		// Dequeuing logic
		curr := queue[0]
		c_ro, c_col := curr[0], curr[1]
		if c_ro < 0 || c_ro >= R || c_col < 0 || c_col >= C {
			break
		}

		// Visiting Logic
		z_pos := c_ro*C + c_col
		if !seen[z_pos] {
			seen[z_pos] = true
			pos_visited += 1
		}

		// Processing next pos
		delta := deltas[dir]
		n_ro, n_col, n_dir := delta[0]+c_ro, delta[1]+c_col, delta[2]
		if n_ro < 0 || n_ro >= R || n_col < 0 || n_col >= C {
			break
		} else if maze[n_ro][n_col] == '#' {
			dir = n_dir
			n_ro, n_col = deltas[n_dir][0]+c_ro, deltas[n_dir][1]+c_col
		}

		queue = append(queue[1:], [2]int{n_ro, n_col})
	}

	fmt.Printf("Positions Visited: %d\n", pos_visited)
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetMaze(f *os.File) ([]string, [2]int) {
	var out []string
	s_pos := [2]int{-1, -1}
	scanner := bufio.NewScanner(f)
	s_regex := regexp.MustCompile(`\^`)
	l_idx := 0
	for scanner.Scan() {
		line := scanner.Text()
		out = append(out, line)
		match := s_regex.FindStringIndex(line)
		if match != nil {
			s_pos[0] = l_idx
			s_pos[1] = match[0]
		}
		l_idx += 1
	}
	return out, s_pos
}
