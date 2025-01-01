package day21

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Coord struct {
	x, y int
}

type Pair struct {
	a, b byte
}

func RunP1() {
	numpad := map[byte]Coord{
		'7': {y: 0, x: 0}, '8': {y: 0, x: 1}, '9': {y: 0, x: 2},
		'4': {y: 1, x: 0}, '5': {y: 1, x: 1}, '6': {y: 1, x: 2},
		'1': {y: 2, x: 0}, '2': {y: 2, x: 1}, '3': {y: 2, x: 2},
		'0': {y: 3, x: 1}, 'A': {y: 3, x: 2},
	}
	dirpad := map[byte]Coord{
		'^': {y: 0, x: 1}, 'A': {y: 0, x: 2},
		'<': {y: 1, x: 0}, 'v': {y: 1, x: 1}, '>': {y: 1, x: 2},
	}
	f := utils.OpenFile(21, false)
	seqs := GetGame(f)

	numGraph := GetAllPairs(numpad, Coord{y: 3, x: 0})
	dirGraph := GetAllPairs(dirpad, Coord{y: 0, x: 0})

	sum := 0
	for _, seq := range seqs {
		l := string(seq)
		ct, _ := strconv.Atoi(l[:len(l)-1])

		s := GetSeq(seq, numGraph)
		s = GetSeq(s, dirGraph)
		s = GetSeq(s, dirGraph)

		sum += ct * len(s)
	}
	fmt.Printf("Sum: %d\n", sum)
}

func GetSeq(charSeq []byte, graph map[Pair][]byte) []byte {
	var out []byte
	prev := byte('A')
	for _, c := range charSeq {
		out = append(out, graph[Pair{prev, c}]...)
		prev = c
	}
	return out
}

func GetAllPairs(keypad map[byte]Coord, invalidCoord Coord) map[Pair][]byte {
	out := make(map[Pair][]byte)
	for k1, v := range keypad {
		for k2, v2 := range keypad {
			x1, y1 := v.x, v.y
			x2, y2 := v2.x, v2.y
			path := []byte{}
			for i := 0; i < x1-x2; i++ {
				path = append(path, '<')
			}
			for i := 0; i < y2-y1; i++ {
				path = append(path, 'v')
			}
			for i := 0; i < y1-y2; i++ {
				path = append(path, '^')
			}
			for i := 0; i < x2-x1; i++ {
				path = append(path, '>')
			}
			p1, p2 := Coord{x1, y2}, Coord{x2, y1}
			if p1 == invalidCoord || p2 == invalidCoord {
				for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
					path[i], path[j] = path[j], path[i]
				}
			}
			path = append(path, 'A')

			out[Pair{k1, k2}] = path
		}
	}
	return out
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetGame(f *os.File) [][]byte {
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out [][]byte
	for scanner.Scan() {
		line := scanner.Text()
		out = append(out, []byte(line))
	}
	return out
}
