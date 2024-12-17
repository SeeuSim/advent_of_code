package day09

import (
	"2024/utils"
	"fmt"
	"os"
	"strconv"
)

func RunP1() {
	f := utils.OpenFile(9, false)
	diskPart, empty := GetPuzzle(f)
	origLen := len(diskPart)

	var updated []DiskPart
	for len(empty) > 0 {
		lMostEmpt := empty[0]
		if lMostEmpt.pos >= origLen {
			break
		}
		empty = empty[1:]

		rMost := diskPart[len(diskPart)-1]
		diskPart = diskPart[:len(diskPart)-1]
		updated = append(updated, DiskPart{
			fNo:     rMost.fNo,
			pos:     lMostEmpt.pos,
			isEmpty: false,
		})
	}
	s := 0
	for _, v := range append(diskPart, updated...) {
		s += v.fNo * v.pos
	}
	fmt.Printf("Sum: %d\n", s)
}

func RunP2() {
	// TODO: Implement Part 2
}

type DiskPart struct {
	fNo     int
	pos     int
	isEmpty bool
}

func (d DiskPart) String() string {
	if d.isEmpty {
		return fmt.Sprintf("{[E]%d}", d.pos)
	}
	return fmt.Sprintf("{[%d]%d}", d.fNo, d.pos)
	// return fmt.Sprintf("{Fno: %d, Pos: %d, isEmpty: %t}", d.fNo, d.pos, d.isEmpty)
}

func GetPuzzle(f *os.File) ([]DiskPart, []DiskPart) {
	defer f.Close()

	isSpace := false
	currFno := 0
	pos := 0
	var normal []DiskPart
	var empty []DiskPart
	contents, _ := os.ReadFile(f.Name())
	for _, c := range contents {
		nPos, _ := strconv.Atoi(fmt.Sprintf("%c", c))
		for nPos > 0 {
			newPart := DiskPart{currFno, pos, isSpace}
			if isSpace {
				empty = append(empty, newPart)
			} else {
				normal = append(normal, newPart)
			}
			nPos -= 1
			pos += 1
		}
		if isSpace {
			currFno++
		}
		isSpace = !isSpace
	}
	return normal, empty
}
