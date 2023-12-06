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

## Day 4 - Scratchcards

### Part 1 - Points from winning sets

Given a line in the below format:

```text
Card   1:  8 86 59 90 68 52 55 24 37 69 | 10 55  8 86  6 62 69 68 59 37 91 90 24 22 78 61 58 89 52 96 95 94 13 36 81
```

The task was to parse all the numbers on the left as a set of winning numbers, then for each line calculate:

- How many cards on the right of the pipe ("|") lie in this set

- For the number of winning cards, calculate the number of points, with:

  - 0 getting no points,

  - 1 onwards getting $2^{\text{count}-1}$ points.

This was trivial enough to calculate, and obtain the sum of all points obtained. 

### Part 2 - Number of cards won

For each of the cards in part 1, each of the cards are already obtained by the player
as 1 copy each.

If, for instance, Card 1 wins 10 numbers, then the player gets 1 additional copy of
the next 10 cards (Card 2 and so on).

Since now we have 2 copies of Card 2, if Card 2 wins 2 numbers, then each of the 
copies of Card 2 now win 2 copies of the next 2 cards (Cards 3 and 4), for a total
of 4 additional cards.

We are now tasked with calculating the number of cards the player ends up with.

Taking inspiration from Christopher Biscardi again, we initialise the count
of each card as 1 in a `BTreeMap` as we are guaranteed to traverse the indices
from low to high always,

and we use the cards won from part 1 to modify the counts in this counter map.

We also increment each of the subsequent cards won by the number of cards the current card has (in the case of Card 2 above, 2).

## Day 5 - Planting Seeds (Range Queries)

### Part 1 - Mapping Singular Values and finding minimum

Given a set of range maps each with a set of entries, like so (this is one range
map):

```text
3305253869 1699909104 39566623
3344820492 1130725752 384459310
3244681427 1739475727 60572442
951517531 1800048169 868898709
1820416240 951517531 179208221
1999624461 2668946878 219310925
3729279802 1515185062 184724042
2218935386 2898481077 1015522767
3234458153 2888257803 10223274
```
For each line, it is formatted in the format (`dest_start`, `source_start`, `length`).

This implies that the mapping of any value in the range $[\text{source\_start, source\_start+length})$ 
will be mapped to the range $[\text{dest\_start, dest\_start+length})$ in a 1:1 mapping.

For instance, if we have (2, 5, 3), we would have 5 mapped to 2, 6 mapped to 3 and 7
mapped to 4 for a length of 3 elements. 

Any elements that fall outside any of the source ranges will be mapped to themselves
in the output.

This constitutes one set of mappings, with each number from a **source** to a
**destination**.

---

After a set of source-to-destination mappings, we arrive at our final mapped numbers
and return the minimum value.

This is trivial enough to perform the iterative mapping for each seed value.

### Part 2 - Mapping a Set of Ranges and finding minimum value

This adapts Part 1 by converting each pair of seeds into: $(\text{seed\_range\_start},\text{range\_length})$.

Then, for each number in the range, find the mappings as per Part 1 and find the minimum
output number.

This is not practical as the ranges can number in the billions. Hence, an approach is needed where we process each set in the following manner, for each input seed range:

- For each mapping in the set, find the intersection of the mapping with the
  input seed range. This entire range can be mapped.

- Find the portion of the input seed range that does not intersect. This range
  maps to itself.

- Take all the mapped ranges and dedupe them so that they do not intersect.

We repeat this for all the sets of mappings, and then find the minimum range by
start point.

This was inspired by GitHub user @sergiocarvalho-soomacom.

## Day 6

### Part 1

This simply maps for each time total, a range from 1 to the time, subtract 
the index and multiply it by the remaining time for the distance travelled.

As the input numbers are small, no optimisations are needed.

### Part 2

As the number is less than 1 billion, the same approach can be taken with
little to no performance penalty.

### Optimisations

There can be optimisations, but they are only really necessary with larger
numbers.

[[ TO BE EXPANDED ]]
