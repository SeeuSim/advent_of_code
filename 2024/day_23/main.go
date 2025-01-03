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
	f := utils.OpenFile(23, false)
	network, nodes := GetGame(f)
	mc := MaxClique(make(map[string]struct{}), nodes, make(map[string]struct{}), network)

	// Format result
	s := []string{}
	for k := range mc {
		s = append(s, k)
	}
	sort.Strings(s)
	fmt.Println("Password:", strings.Join(s, ","))
}

func MaxClique(R, P, X map[string]struct{}, graph map[string]map[string]struct{}) map[string]struct{} {
	if len(P) == 0 && len(X) == 0 {
		return R
	}

	maxClique := make(map[string]struct{})
	for n := range CopySet(P) {
		c := MaxClique(Union(R, map[string]struct{}{n: {}}), Intersect(P, graph[n]), Intersect(X, graph[n]), graph)
		if len(c) > len(maxClique) {
			maxClique = CopySet(c)
		}
		delete(P, n)
		X[n] = struct{}{}
	}
	return maxClique
}

func CopySet(s map[string]struct{}) map[string]struct{} {
	out := make(map[string]struct{})
	for k := range s {
		out[k] = struct{}{}
	}
	return out
}

func Union(a, b map[string]struct{}) map[string]struct{} {
	out := make(map[string]struct{})
	for k := range a {
		out[k] = struct{}{}
	}
	for k := range b {
		out[k] = struct{}{}
	}
	return out
}

func Intersect(a, b map[string]struct{}) map[string]struct{} {
	out := make(map[string]struct{})
	for k := range a {
		if _, e := b[k]; e {
			out[k] = struct{}{}
		}
	}
	return out
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
