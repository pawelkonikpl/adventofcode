package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Data struct {
	Left  string
	Right string
}

type Custom map[string]int

var WinningTable map[string]Custom

func calculatePoints(leftRight []Data) int {
	points := 0
	sum := 0

	for _, data := range leftRight {
		if data.Right == "X" {
			points = 1
		}
		if data.Right == "Y" {
			points = 2
		}
		if data.Right == "Z" {
			points = 3
		}
		winningPoints := WinningTable[data.Left][data.Right]

		points = points + winningPoints
		sum += points
	}

	return sum

}

func readFile() []Data {
	path, err := os.Getwd()
	if err != nil {
		log.Panicf(err.Error())
	}
	readFile, err := os.Open(path + "/2022/day2/data.txt")
	defer readFile.Close()

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)
	var data []Data

	for fileScanner.Scan() {
		line := fileScanner.Text()
		res := strings.Split(line, " ")

		li := Data{
			Left:  res[0],
			Right: res[1],
		}
		data = append(data, li)
	}
	return data
}

func main() {
	WinningTable = map[string]Custom{
		"A": {
			"X": 3,
			"Y": 6,
			"Z": 0,
		},
		"B": {
			"X": 0,
			"Y": 3,
			"Z": 6,
		},
		"C": {
			"X": 6,
			"Y": 0,
			"Z": 3,
		},
	}
	data := readFile()
	sum := calculatePoints(data)
	log.Println(sum)
}
