package utils

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func OpenFile(day int, isTest bool) *os.File {
	var path strings.Builder
	path.WriteString("./day_")
	path.WriteString(fmt.Sprintf("%02d/", day))
	if isTest {
		path.WriteString("test.in")
	} else {
		path.WriteString("input.in")
	}

	f, e := os.Open(path.String())
	if e != nil {
		log.Fatalf("Error occurred opening file: %v\n", e)
		os.Exit(1)
		return nil
	}
	return f
}
