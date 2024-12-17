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
	f := utils.OpenFile(9, false)
	files, empty := GetPuzzleP2(f)
	for i := len(files) - 1; i >= 0; i-- {
		rMostFile := files[i]
		emptyIdx := -1
		for eIdx, empt := range empty {
			if empt.sPos > rMostFile.sPos {
				break
			} else if empt.len < rMostFile.len {
				continue
			}
			emptyIdx = eIdx
			break
		}
		if emptyIdx < 0 {
			continue
		}
		empty[emptyIdx].len -= rMostFile.len
		files[i].sPos = empty[emptyIdx].sPos
		if empty[emptyIdx].len == 0 {
			empty = append(empty[:emptyIdx], empty[emptyIdx+1:]...)
		} else {
			empty[emptyIdx].sPos += rMostFile.len
		}

	}

	s := 0
	for _, f := range files {
		for i := 0; i < f.len; i++ {
			s += f.fNo * (f.sPos + i)
		}
	}
	fmt.Printf("Sum: %d\n", s)
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

type File struct {
	fNo     int
	sPos    int
	len     int
	isEmpty bool
}

func (f File) String() string {
	if f.isEmpty {
		return fmt.Sprintf("{[E]<%d...%d>}", f.sPos, f.sPos+f.len-1)
	}
	return fmt.Sprintf("{[%d]<%d...%d>}", f.fNo, f.sPos, f.sPos+f.len-1)
}

func GetPuzzleP2(f *os.File) ([]File, []File) {
	defer f.Close()

	contents, _ := os.ReadFile(f.Name())
	var files []File
	var empty []File
	cPos := 0
	isSpace := false
	for i, c := range contents {
		nPos, _ := strconv.Atoi(fmt.Sprintf("%c", c))
		f := File{
			i / 2,
			cPos,
			nPos,
			isSpace,
		}
		if isSpace {
			empty = append(empty, f)
		} else {
			files = append(files, f)
		}
		cPos += nPos
		isSpace = !isSpace
	}
	return files, empty
}
