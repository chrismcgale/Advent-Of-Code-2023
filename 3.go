package main

import (
    "fmt"
    "io/ioutil"
	"unicode"
	"strings"
	"strconv"
)

type Key struct {
    X int
    Y int
}

func find_number(runes []rune, i int, j int, visited map[Key]bool) (int) {
	if visited[Key{i, j}] {
		return 0
	}
	 
	visited[Key{i, j}] = true
	buffer := []rune{runes[j]}

	var curr = j - 1

	for {
		if curr < 0 || !unicode.IsDigit(runes[curr]) {
			break
		}
		if visited[Key{i, curr}] {
			return 0
		}
		 
		visited[Key{i, curr}] = true
		buffer = append([]rune{runes[curr]}, buffer...)
		curr -= 1
	}


	curr = j + 1

	for {
		if curr >= len(runes) || !unicode.IsDigit(runes[curr]) {
			break
		}
		if visited[Key{i, curr}] {
			return 0
		}
		 
		visited[Key{i, curr}] = true
		buffer = append(buffer, runes[curr])
		curr += 1
	}

	var result = 0

	for k := 0; k < len(buffer); k++ {
		add, _ := strconv.Atoi(string(buffer[k]))
		result = result * 10 + add
	}

	fmt.Println(i, j, result)

	return result

}

func main() {
    content, err := ioutil.ReadFile("3.input")
    if err != nil {
        fmt.Println("Error reading file:", err)
        return
    }

	var lines = strings.Split(string(content), "\n")

	var sum = 0

	var visited = make(map[Key]bool)

	for i := 0; i < len(lines); i++ {
		for j := 0; j < len(lines[i]); j++ {
			runes := []rune(lines[i])
			if lines[i][j] != '.' && lines[i][j] != '\n' && !unicode.IsDigit(runes[j]) {
				// Last row
				if i > 0 {
					last_runes := []rune(lines[i - 1])
					if unicode.IsDigit(last_runes[j]) {
						sum += find_number(last_runes, i - 1, j, visited)
					} else {
						if j - 1 >= 0 && unicode.IsDigit(last_runes[j - 1]) {
							sum += find_number(last_runes, i - 1, j - 1, visited)
						}

						if j + 1 < len(lines[i - 1]) && unicode.IsDigit(last_runes[j + 1]) {
							sum += find_number(last_runes, i - 1, j + 1, visited)
						}
					}
				}
				// My row
				if j - 1 >= 0 && unicode.IsDigit(runes[j - 1]) {
					sum += find_number(runes, i, j - 1, visited)
				}

				if j + 1 < len(lines[i]) && unicode.IsDigit(runes[j + 1]) {
					sum += find_number(runes, i, j + 1, visited)
				}

				// next row
				if i < len(lines) - 1 && j < len(lines[i + 1]) {
					next_runes := []rune(lines[i + 1])
					if unicode.IsDigit(next_runes[j]) {
						fmt.Println(i, j)
						sum += find_number(next_runes, i + 1, j, visited)
					} else {
						if j - 1 >= 0 && unicode.IsDigit(next_runes[j - 1]) {
							sum += find_number(next_runes, i + 1, j - 1, visited)
						}

						if j + 1 < len(lines[i + 1]) && unicode.IsDigit(next_runes[j + 1]) {
							sum += find_number(next_runes, i + 1, j + 1, visited)
						}
					}
				}

				
			}

		}
	}

    fmt.Println(sum)
}
