# Advent of Code 2023

**Author**: Ong Seeu Sim

My attempt repository for this year's [Advent of Code](https://adventofcode.com/2023).
I decided to attempt it in Rust, so all the source code here is in Rust 🤠

## 📚 Table of Contents

- [Day 1 - Trebuchet](#day-1---trebuchet)

  - [Part 1 - Simple Digits](#part-1---simple-digits)

  - [Part 2 - Digits with Words](#part-2---digits-including-words-and-overlapping-ones)

- [Day 2 - Cube Conundrum](#day-2---cube-conundrum)

  - [Part 1 - Valid Games](#part-1---indexes-of-valid-games)

  - [Part 2 - Sum of products of minimum cubes needed for a game](#part-2---product-of-minimum-cubes-needed-for-a-game)

- [Day 3 - Gear Ratios](#day-3---gear-ratios)

  - [Part 1 - Parts adjacent to component](#part-1---sum-all-parts-adjacent-to-components)

  - [Part 2 - Gears adjacent to two parts](#part-2---sum-of-all-gear-ratios-of-gears-adjacent-to-two-parts-only)

## Day 1 - Trebuchet

### Part 1 - Simple Digits

Given a list of strings ([file](/src/day_1/input.txt)), parse the leftmost **singular** digit (0 - 9),
and the rightmost **singular** digit (0-9) in each line, and combine them into a number like so:

```rs
use std::fmt::format;

let first_digit = 1;
let last_digit = 2;
let num = format!("{}{}", first_digit, last_digit)
    .parse::<u32>()
    .unwrap(); // 12
  ```

In the event that the line only has one digit, that digit is the leftmost AND the rightmost digit.

Take the sum of all the numbers formed in this manner, for each line.

### Part 2 - Digits Including Words (and overlapping ones)

An extension of the above, just that now, any **lowercase** English alphabets that spell out numbers
(i.e. 'one', 'two', ..., 'nine') also form valid digits.

This proved quite tricky, since strings like these: "twoneightwone" have overlapping numbers that 
are hard to find with regular regex capture groups.

I initially attempted to use the `fancy_regex` crate to provide pattern matching for reverse lookups
(i.e. `r"(?<=(one))`) and so on, but after watching 
[Christopher Biscardi's video](https://youtu.be/JOgQMjpGum0?si=u-UR2ILMLUbJJuux),
I discovered that Rust iterators provide a faster lookup (by 1000x) by simply performing:

- a sliding window on the line with step of 1 and
- comparing if each window starts with the "one", "two", etc. or parsing the first char as a digit.

It also makes use of the `filter_map` iterator function to filter out results which resulted in Errors
from the parsing of the first char as a digit where:

- the slice did not start with a lowercase spelling of a digit in English.

## Day 2 - Cube Conundrum

### Part 1 - Indexes of valid games

Given a list of strings ([file](/src/day_2/input.txt)), each line is made up of a pattern
representing a game like so:

`Game {index}: x red, y blue, z green; x_0 red, y_0 blue, z_0 green; ...`

Each game consists of a set of values (`x red, y blue, z green`) separated by a semicolon (';').
In each set, if:

- the number of red exceeds 12, or

- the number of blue exceeds 13, or

- the number of green exceeds 14,

then that set indicates that the game is not possible and that line is invalid.

The goal is to sum up the indices (1 indexed) of all the valid lines.

### Part 2 - Product of minimum cubes needed for a game

This adapts from Part 1 by processing the minimum number of cubes (red, blue, green) needed
for each set so that the number of cubes for each color in each set does not exceed this minimum
number.

Then, take the product of the three minimum numbers for each line, and sum up all the lines.

This involved a bit more processing to keep it within the Rust iterators, so I used a HashMap
to:

- accumulate the maximum value seen for each cube color across the sets for each line, and

- take their values as an iterable and fold them multiplicatively, and

- fold all the products across the lines into one sum.

## Day 3 - Gear Ratios

### Part 1 - Sum all parts adjacent to components

Given a list of strings ([file](/src/day_3/input.txt)), each line is made up of a pattern
representing a game like so:

```txt
...122..................*.....*..........
....+..........259....698..373.992.52.674
```

In each line, there are some numbers representing part numbers. If a part is adjacent to
a component (any non-'.' character), then take its part number.

Accumulate the sum of all part numbers.

I used a simple approach to check all neighboring characters around a captured number, and
if it was valid, I took its value.

Else, I took a value of 0 to represent not taking the number, and I summed up all the numbers
in this manner.

### Part 2 - Sum of all gear ratios of gears adjacent to two parts only

This adapts from part 1 by taking all gears "*" which have only 2 parts that are adjacent to them
as per the rules above, and multiplying their part numbers to form the gear ratio.

Then, sum up all the gear ratios of these gears to form the answer.

I adapted the approach from part 1 to find all gears a part was adjacent to, and memoize the
part under a list of parts for each gear in the list.

I then filtered out the gears which only had 2 adjacent parts, multiplied their part numbers to
form the ratio, and took the sum.

Also, as I had to repeatedly access line character indices to check them across neighboring lines,
I had to extract out the line characters from their iterators into memory to allow repeated access
as opposed to using the `chars()` iterator, which only allows access to an index **once**.
