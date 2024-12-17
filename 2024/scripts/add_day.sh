#!/bin/sh

largest=$(ls | grep -E '^day_[0-9]{2}' | sed 's/day_//' | sort -n | tail -n 1)

if [[ -z "$largest" ]]; then
  largest=0
fi

new_day=$((10#$largest + 1))
new_day_formatted=$(printf "%02d" "$new_day")

path="day_$new_day_formatted"

if [[ -d "$path" ]]; then
  echo "Folder with path $path exists. Exiting..."
  exit 1
fi

mkdir $path
if [[ -d "$path" ]]; then
  echo "package day$new_day_formatted

func RunP1() {
    // TODO: Implement Part 1
}

func RunP2() {
    // TODO: Implement Part 2
}
" > "$path/main.go"

  touch "$path/input.in"
  touch "$path/test.in"
fi
