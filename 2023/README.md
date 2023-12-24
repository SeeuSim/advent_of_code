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

- [Day 4 - Scratch Cards](#day-4---scratchcards)
  - [Part 1 - Points from Winning Sets](#part-1---points-from-winning-sets)
  - [Part 2 - Number of Cards Won](#part-2---number-of-cards-won)

- [Day 5 - Planting Seeds Range Queries](#day-5---planting-seeds-range-queries)
  - [Part 1 - Mapping Singular Values and Finding Minimum](#part-1---mapping-singular-values-and-finding-minimum)
  - [Part 2 - Mapping A Set Of Ranges and Finding Minimum value](#part-2---mapping-a-set-of-ranges-and-finding-minimum-value)

- [Day 6 - Wait For It](#day-6---wait-for-it)
  - [Part 1 - Basic Time Trial per Race](#part-1---basic-time-trial-per-race)
  - [Part 2 - Accumulating all values into one](#part-2---accumulating-all-values-into-one)

- [Day 7 - Camel Cards](#day-7---camel-cards)
  - [Part 1 - Parsing of hands and comparing them](#part-1---parsing-of-hands-and-comparing-them)
  - [Part 2 - Comparing hands with joker swaps](#part-2---comparing-hands-with-joker-swaps)

- [Day 8 = Haunted Wasteland](#day-8---haunted-wasteland)
  - [Part 1 - Steps from AAA to ZZZ](#part-1---counting-steps-to-reach-zzz-from-aaa-for-a-single-node)
  - [Part 2 - Synchronous Steps for all ending with 'A' to reach destination ending with 'Z'](#part-2---counting-steps-for-multiple-nodes-to-reach-destination-ending-with-z-simultaneously)

- [Day 9 - Mirage Maintenance](#day-9---mirage-maintenance)
  - [Part 1 - Extrapolate Forwards](#part-1---extrapolate-forwards)
  - [Part 2 - Extrapolate Backwards](#part-2---extrapolate-backwards)

- [Day 10 - Pipe Maze](#day-10---pipe-maze)
  - [Part 1 - Furthest Point](#part-1---furthest-point)
  - [Part 2 - Area Within](#part-2---area-within)
  
- [Day 11 - Cosmic Expansion](#day-11---cosmic-expansion)
  - [Part 1 - Expanding Manually](#part-1---expanding-manually-and-sum-of-all-pairs-shortest-paths)
  - [Part 2 - Expanding using Math](#part-2---expanding-using-math-and-sum-of-all-pairs-shortest-paths)

- [Day 12 - Hot Springs](#day-12---hot-springs)
  - [Part 1 - Number of ways to fit configuration into springs](#part-1---number-of-ways-to-fit-configuration-into-springs)
  - [Part 2 - Increasing pattern length](#part-2---increasing-pattern-length)

- [Day 13 - Point Of Incidence](#day-13---point-of-incidence)
  - [Part 1 - Find horizontal or vertical line of reflection](#part-1---find-horizontal-or-vertical-line-of-reflection)
  - [Part 2 - Find line with one replacement](#part-2---find-line-with-one-replacement)

- [Day 14 - Parabolic Reflector Dish](#day-14---parabolic-reflector-dish)
  - [Part 1 - Tilting North and Counting score](#part-1---tilting-north-and-counting-score)
  - [Part 2 - Tilting in all directions, for 1,000,000,000 times](#part-2---tilting-in-all-directions-for-1000000000-times)

- [Day 15 - Lens Library](#day-15---lens-library)
  - [Part 1 - Obtain Hash Codes](#part-1---obtain-hash-codes)
  - [Part 2 - Place Lens Filters and Calculate scores](#part-2---place-lens-filters-and-calculate-scores)

- [Day 16 - The Floor Will Be Lava](#day-16---the-floor-will-be-lava)
  - [Part 1 - Calculating Visited Cells in Grid](#part-1---calculating-visited-cells-in-grid)
  - [Part 2 - Calculating Optimal Start Point for Grid](#part-2---calculating-optimal-start-point-for-grid)

- [Day 17 - Clumsy Crucible](#day-17---clumsy-crucible)
  - [Part 1 - Shortest Path with max 3 consec. jumps per direction](#part-1---shortest-path-with-max-3-consec-jumps-per-direction)
  - [Part 2 - Minimum 4, Maximum 10 jumps per direction](#part-2---minimum-4-maximum-10-jumps-per-direction)

- [Day 18 - Lavaduct Lagoon](#day-18---lavaduct-lagoon)
  - [Part 1 - Parsing values from left](#part-1---parsing-values-from-left)
  - [Part 2 - Parsing values from end](#part-2---parsing-values-from-end)

- [Day 19 - Aplenty](#day-19---aplenty)
  - [Part 1 - Processing each part individually](#part-1---processing-each-part-individually)
  - [Part 2 - Processing all possible combinations of ranges](#part-2---processing-all-possible-combinations-of-ranges)

- [](#day-x)
  - [](#part-1)
  - [](#part-2)

<!-- - [](#day-x)
  - [](#part-1)
  - [](#part-2) -->


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

## Day 6 - Wait For It

### Part 1 - Basic time trial per race

This simply maps for each time total, a range from 1 to the time, subtract 
the index and multiply it by the remaining time for the distance travelled.

As the input numbers are small, no optimisations are needed.

### Part 2 - Accumulating all values into one

As the number is less than 1 billion, the same approach can be taken with
little to no performance penalty.

### Optimisations

There can be optimisations, but they are only really necessary with larger
numbers.

## Day 7 - Camel Cards

### Part 1 - Parsing of hands and comparing them

This requires for hands to be parsed, and compared against one another.
I opted to store the hands as a struct, containing:

1. Their classified kind, for comparison

2. Their raw hand values, for disputing

3. Their bets, for calculating the sum.

This now simply requires the implementation of the parsing logic,
the comparison logic, as well as the collecting of results.

#### Parsing Logic

We need to parse the string, as well as classify it.

We implement the `from_str` method to parse it, and depending
on the different char counts, classify the hand type.

#### Comparator Logic

We simply compare the kind, and if the hand kind matches,
we compare the character order index by index.

### Part 2 - Comparing hands with Joker swaps

This alters the parsing and classification logic, to classify
the hand after taking the Joker, if any, to improve the hand
value.

We then modified the `from_str` method to take this hand kind
classification into account.

This also alters the comparison logic by reducing the index of the
joker to the lowest. We simply swapped the order to prioritise
Joker to the lowest value.

## Day 8 - Haunted Wasteland 

### Part 1 - Counting steps to reach 'ZZZ' from 'AAA' for a single node 

Given a graph with nodes containing 'left' and 'right' pointers, calculate
the number of steps required, following a cyclic set of instructions
'L' or 'R' to reach the 'ZZZ' node from 'AAA',

This is a simple linked-list traversal that takes less than 2^15 steps.

### Part 2 - Counting steps for multiple nodes to reach destination ending with 'Z' _simultaneously_ 

As this changes the start node to all nodes ending with 'A', there are multiple nodes
to take into consideration.

As each node takes a different number of steps to reach the first destination node ending
with 'Z', we can first investigate with two of the nodes.

As it turns out, for the first two nodes, they both reach a node ending with 'Z' when the
step count reaches the lowest common multiple of their individual step counts.

Investigating this, combining the first node with each of the other nodes ending with 'A',
we find that this relation holds true.

Hence, we combine all the step counts into their LCM and obtain the answer.

## Day 9 - Mirage Maintenance

### Part 1 - Extrapolate forwards

For this part, the goal was to take each line,
a sequence of numbers, and generate new lines
each made up of the pairwise difference of 
adjacent elements of the line above, like so:

```text
1 2 3 4 5
 1 1 1 1
  0 0 0
```

In this case, the original line is the first,
and the desired extrapolated values can
be obtained by summing the entire right edge 
formed by '5', '1' and '0'.

This was easily doable with iterators and vector
operations.

The answer was the sum of all extrapolated values.

### Part 2 - Extrapolate backwards

This extends from part 1 by extrapolating backwards
and predicting the value preceding the first.

In the example in Part 1, it is clear that the 
extrapolated value is 0.

The adjustment made was to take a rightwards fold
of all sequences generated, subtracting the accumulated
value from the next.

Using the above diagram in Part 1 as an example,
it would start from the first element of the last line
('0'), taking $1 - 0 = 1$ for the next line, and $1 - 1 = 0$ for the
initial line to obtain the final value of 0.


## Day 10 - Pipe Maze 

### Part 1 - Furthest Point 

Using a loop and the direction pointers, I looped around until
I ended up back at 'S'.

Using the total iteration count divided by 2, would obtain the furthest
distance along the loop from 'S'.

### Part 2 - Area Within 

This was slightly more involved.

Using the same logic as earlier, I kept track of all visited positions
along the loop.

Then, I iterated over each character, to iterate from the beginning,
how many inversions of the loop there were.

An inversion occurs when a vertical pipe is encountered, or a 'J' or 'L' is encountered.
These signify either exiting a west-north bend, entering the loop, or entering a
north-east bend, going along a straight and possibly out of the loop.

This gives some slight over-counting by 6. I verified this using other solutions
online, like this one: [link](https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day10p2.py),
and implemented that logic in the function `pipe_maze_two`, which I will explain [below](#working-solution).

I left the original, broken, solution in the function `broken_pipe_maze_two`.

#### Working Solution

It functions largely similarly to the original, just replacing the 'S' with its possible characters,
and replacing all non-loop characters with '.' to minimise confusion.

Then, to count inversions by crossing the vertical characters, it iterates over each row, finding
vertical characters and marking as within/not-within the loop.

It also avoids the 'L----J' and 'L-----7' branches by setting a flag if a 'L' or 'F' pipe has first
been encountered, to count if the ending character of the bend is a 'J' or a '7', and hence if to
count the crossing of the last character as an inversion or not.

If for instance it is an 'L----J', then effectively two inversions have been made and nothing changes.
Else, only one inversion has been made, and the state storing if the chars are within the loop can
be inverted.

Any time it is not within the loop, the characters are added to the 'outside' set, and the union
of the 'outside' set with the pipe loop set gives the total number of characters that are pipes
or not in the loop.

Subtracting the total number of characters from this union gives the total number of characters
that are within the loop.

## Day 11 - Cosmic Expansion

### Part 1 - Expanding manually and sum of all pairs shortest paths

Simply iterate through each line, doubling the rows where needed and
doubling columns (from reverse to keep indices intact) where needed as well.

Then, I iteratively used pairings of each galaxy to calculate the row and
column difference between their indices for the shortest path sum.

### Part 2 - Expanding using math and sum of all pairs shortest paths

Since the row expansion factor was by a million, this could potentially
slow the calculations down.

Hence, I used manual calculations to calculate and insert a million
rows/columns for each expanded row/column per path sum.

## Day 12 - Hot Springs

### Part 1 - Number of ways to fit configuration into springs

This involved iterating down each line with pointers across two iterators,
the first being the line itself, and the second being the groups of springs.

### Part 2 - Increasing pattern length

This involved simply repeating the pattern from part 1 5 times, and increasing
the output range from i32 to i64.

## Day 13 - Point of Incidence

### Part 1 - Find horizontal or vertical line of reflection

Iterating through the pattern, starting from different start columns or rows,
for vertical and horizontal reflections respectively, we compare each character
with its reflection about the line of reflection.

If we find a line index which causes a valid reflection, then that line index is
taken as the line of reflection, and the score is as such.

### Part 2 - Find line with one replacement

This iterates from the previous part by first obtaining the original score,
and using the coordinates from the previous part as a banned set of positions,
to iterate through each row and column, replacing each character and finding the
first alternate line of reflection.

This uses a sort of 'backtracking' approach to make the modification and backtrack
once the calculation is made.

## Day 14 - Parabolic Reflector Dish

### Part 1 - Tilting North and Counting score

For this part, I took inspiration from hyper neutrino again, and implemented a
simple function to tilt the grid north.

Then, I used another function to simply count the number of 'O's in each row
and get the total score for that row, summing up the scores from all the rows.

### Part 2 - Tilting in all directions, for 1,000,000,000 times

TO BE IMPLEMENTED

Disclaimer: I used hyperneutrino's solution for this, and am still not entirely
sure how to obtain the actual correct solution using Rust.

## Day 15 - Lens Library

### Part 1 - Obtain Hash Codes

For this, all that was needed was some folding logic to calculate the hash
score according to specifications.

### Part 2 - Place Lens Filters and Calculate scores

For this, a bit more iteration was needed, but by simply pre-allocating
the vector of boxes and inserting/updating/removing the lenses as specified,
the score is obtained by multiplying the box index with the refractive index
of each lens, their position within the box, and summing them all up.

## Day 16 - The Floor Will Be Lava

### Part 1 - Calculating Visited Cells in Grid

For this part, the answer is simply the total number of visited cells in the grid,
by exploring the grid.

Each cell may be explored up to 1 time in each of the directions.

However, as the answer is only keen on counting the total number of explored cells,
we strip the visited delta from each cell and only count the total number of unique cells.

### Part 2 - Calculating Optimal Start Point for Grid

For this part, we use the score obtaining function from part 1 and iterate through all
possible starting positions to obtain the maximum score.

## Day 17 - Clumsy Crucible

### Part 1 - Shortest Path with max 3 consec. jumps per direction

For this, a simple Djikstra's algorithm with minimum heap was needed to
implement the shortest path search.

A custom struct with ordering was needed to work effectively with the
binary heap struct.

Then, custom loop logic was needed for each iteration to add forward
jumps only if there were allowance (less than 3) remaining in that
direction.

Jumps in other directions were also needed to kickstart the exploration
of paths in other directions.

By virtue of the priority queue, nodes with the shortest total path cost
will be explored first and it is guaranteed that the first time the 
destination cell is hit, the cost returned is the lowest.

### Part 2 - Minimum 4, Maximum 10 jumps per direction

This simply altered the logic of the previous part by tweaking the forward
exploration limit to 10 jumps, and the exploration of other directions to
only the start node, as well as when at least 4 jumps were made.

## Day 18 - Lavaduct Lagoon

### Part 1 - Parsing values from left

For this part, parsing the values from the left of the pattern was
relatively straightforward.

To form the polygon vertices, it suffices to iterate through the
instructions, multiplying the deltas by the distance of each instruction,
then accumulating the coordinates to get the next vertice coordinate.

Then, to get the polygon area, it suffices to use the shoelace method
to obtain the polygon area.

However, as the polygon is not a regular shape, this eats into the
circumference. Hence, we use Pick's theorem to obtain the internal area
comprising all internal integers of the polygon.

Then, adding the circumference, we obtain the total area.

### Part 2 - Parsing values from end

To simplify parsing the hex values from the end of each instruction,
we used a regex capture to capture only the needed characters, then
split them, taking the last character as the direction, while the front
5 characters as a hex digit using the `u64::from_str_radix` function.

As the values are relatively larger, annotation of types to `u64` and `i64`
are needed to prevent overflow.

As we used a mathematical method for the previous part, no space constraints
were needed for this one.

## Day 19 - Aplenty

### Part 1 - Processing each part individually

Given a list of workflows, I defined a custom struct for each workflow, and
a custom struct for each Rule for a separation of concerns on which struct
handled what responsibility.

I also designed a Part struct as a logical grouping for the JSON object with 
'x', 'm', 'a', and 's' keys.

Using the parsing logic, and providing a `process` function on each workflow
to evaluate each `Part`, I was able to quickly classify if each part was
to be accepted or not, and perform the relevant sums.

### Part 2 - Processing all possible combinations of ranges

This was slightly trickier as there was a large number of ranges, to the
${10}^{14}$, and processing each combination of values
would be rather costly.

Hence, I used a set of range computations, and partitioned
them according to pass or fail ranges for each rule.

Then, for those that got passed and redirected, I recursively
processed them until they ended in rejected or accepted ranges.

Using this base state, I summed up all the possible counts.

Given the limited set of rules, the depth of this recursion
tree was not too deep, and the computation was near instantaneous.

## Day X -

### Part 1 -

### Part 2 -
<!-- ## Day X -

### Part 1 -

### Part 2 - -->


[[ TO BE EXPANDED ]]