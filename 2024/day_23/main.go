package day23

import (
	"2024/utils"
	"bufio"
	"fmt"
	"os"
	"regexp"
	"sort"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(23, false)
	network, nodes := GetGame(f)
	candidates := []string{}
	for n := range nodes {
		if strings.HasPrefix(n, "t") {
			candidates = append(candidates, n)
		}
	}

	qualifiedTuples := make(map[Tuple]struct{})
	for _, tCand := range candidates {
		for neighbourA := range network[tCand] {
			for neighbourB := range network[neighbourA] {
				if _, e := network[tCand][neighbourB]; e {
					s := []string{tCand, neighbourA, neighbourB}
					sort.Strings(s)
					qualifiedTuples[Tuple{s[0], s[1], s[2]}] = struct{}{}
				}
			}
		}
	}
	fmt.Println("# Qualified:", len(qualifiedTuples))
}

func RunP2() {
	// TODO: Implement Part 2
}

type Tuple struct {
	a, b, c string
}

func GetGame(f *os.File) (map[string]map[string]struct{}, map[string]struct{}) {
	defer f.Close()
	scanner := bufio.NewScanner(f)

	r := regexp.MustCompile(`(.+)-(.+)`)
	network := make(map[string]map[string]struct{})
	nodes := make(map[string]struct{})
	for scanner.Scan() {
		l := scanner.Text()
		m := r.FindAllStringSubmatch(l, -1)[0]
		from, to := m[1], m[2]
		if _, e := network[from]; !e {
			network[from] = make(map[string]struct{})
		}
		network[from][to] = struct{}{}
		if _, e := network[to]; !e {
			network[to] = make(map[string]struct{})
		}
		network[to][from] = struct{}{}
		nodes[from] = struct{}{}
		nodes[to] = struct{}{}
	}
	return network, nodes
}

type NetworkEdge struct {
	from, to string
}
