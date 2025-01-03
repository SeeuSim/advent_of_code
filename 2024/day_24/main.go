package day24

import (
	"2024/utils"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func RunP1() {
	f := utils.OpenFile(24, false)
	initValues, gates := GetGame(f)
	circuit := make(map[string]int)
	for _, l := range initValues {
		SetInitGate(l, circuit)
	}
	g := [][]string{}
	for _, gt := range gates {
		g = append(g, GetGate(gt))
	}
	readyGates, unReadyGates := [][]string{}, [][]string{}
	for _, gt := range g {
		if IsReady(gt, circuit) {
			readyGates = append(readyGates, gt)
		} else {
			unReadyGates = append(unReadyGates, gt)
		}
	}
	for len(readyGates) > 0 {
		for _, g := range readyGates {
			EvalGate(g, circuit)
		}
		readyGates = [][]string{}
		newUnreadyGates := [][]string{}
		for _, g := range unReadyGates {
			if IsReady(g, circuit) {
				readyGates = append(readyGates, g)
			} else {
				newUnreadyGates = append(newUnreadyGates, g)
			}
		}
		unReadyGates = newUnreadyGates
	}

	keys := []string{}
	for k := range circuit {
		if strings.HasPrefix(k, "z") {
			keys = append(keys, k)
		}
	}

	o := 0
	sort.Strings(keys)
	for i := len(keys) - 1; i >= 0; i-- {
		o |= circuit[keys[i]]
		if i > 0 {
			o <<= 1
		}
	}
	fmt.Println("Digit:", o)
}

func RunP2() {
	// TODO: Implement Part 2
}

func GetGame(f *os.File) ([]string, []string) {
	defer f.Close()

	_c, _ := os.ReadFile(f.Name())
	c := string(_c)
	parts := strings.Split(c, "\n\n")
	initValues := strings.Split(parts[0], "\n")
	gates := strings.Split(parts[1], "\n")

	return initValues, gates
}

func SetInitGate(initGate string, circuit map[string]int) {
	p := strings.Split(initGate, ": ")
	key := p[0]
	b, _ := strconv.Atoi(p[1])
	circuit[key] = b
}

func GetGate(gate string) []string {
	p := strings.Split(gate, " ")
	l, r := p[0], p[2]
	op := p[1]
	dest := p[4]
	return []string{l, r, op, dest}
}

func IsReady(gateCombi []string, circuit map[string]int) bool {
	l, r := gateCombi[0], gateCombi[1]
	_, lE := circuit[l]
	_, rE := circuit[r]
	return lE && rE
}

func EvalGate(gateCombi []string, circuit map[string]int) {
	l, r, op, dst := gateCombi[0], gateCombi[1], gateCombi[2], gateCombi[3]
	var out int
	switch op {
	case "XOR":
		out = circuit[l] ^ circuit[r]
	case "AND":
		out = circuit[l] & circuit[r]
	case "OR":
		out = circuit[l] | circuit[r]
	}
	circuit[dst] = out
}
