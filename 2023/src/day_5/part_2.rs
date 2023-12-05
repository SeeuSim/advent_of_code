use crate::utils::extract_file;
use std::io::BufRead;

// Credits @sergiocarvalho-soomacom

pub trait CategoryMapper {
    fn map(&self, value: ValueRange) -> (Vec<ValueRange>, Vec<ValueRange>);
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValueRange {
    pub start: i64,
    pub end: i64,
}
impl ValueRange {
    pub fn from_vec(v: &mut Vec<i64>) -> Vec<ValueRange> {
        let mut result: Vec<ValueRange> = Vec::<ValueRange>::new();
        while !v.is_empty() {
            result.push(ValueRange {
                start: v[0],
                end: v[0] + v[1] - 1,
            });
            *v = v.split_off(2);
        }
        result
    }
    pub fn left_common_right(
        self,
        other: ValueRange,
    ) -> (Option<ValueRange>, Option<ValueRange>, Option<ValueRange>) {
        (
            if other.start < self.start {
                Some(ValueRange {
                    start: other.start,
                    end: other.end.min(self.start - 1),
                })
            } else {
                None
            },
            if other.start <= self.end && other.end >= self.start {
                Some(ValueRange {
                    start: other.start.max(self.start),
                    end: other.end.min(self.end),
                })
            } else {
                None
            },
            if other.end > self.end {
                Some(ValueRange {
                    start: other.start.max(self.end + 1),
                    end: other.end,
                })
            } else {
                None
            },
        )
    }
    pub fn minus(self, other: ValueRange) -> Option<ValueRange> {
        if other.start <= self.start && other.end >= self.end {
            None
        } else if other.end < self.start || other.start > self.end {
            Some(self)
        } else {
            Some(ValueRange {
                start: if other.start <= self.start {
                    other.end + 1
                } else {
                    self.start
                },
                end: if other.start <= self.start {
                    self.end
                } else {
                    other.start - 1
                },
            })
        }
    }
}
pub fn auto_subtract(mut values: Vec<ValueRange>) -> Vec<ValueRange> {
    if values.len() == 0 {
        vec![]
    } else {
        let tail = values.split_off(1);
        let subtracted = tail.iter().fold(Some(values[0]), |left, right| {
            if let Some(left) = left {
                left.minus(*right)
            } else {
                None
            }
        });
        let mut result = if let Some(subtracted) = subtracted {
            vec![subtracted]
        } else {
            vec![]
        };
        result.append(&mut auto_subtract(tail));
        result
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct CategoryMap {
    pub destination: i64,
    pub source: ValueRange,
}
impl CategoryMap {
    pub fn from_str(s: &str) -> CategoryMap {
        let mut iter = s.split(" ").flat_map(str::parse::<i64>);
        let destination = iter.next().unwrap();
        let source = iter.next().unwrap();
        let range = iter.next().unwrap();
        CategoryMap {
            destination: destination,
            source: ValueRange {
                start: source,
                end: source + range - 1,
            },
        }
    }
    pub fn maps(&self, value: i64) -> bool {
        value >= self.source.start && value <= self.source.end
    }
}
impl CategoryMapper for CategoryMap {
    fn map(&self, value: ValueRange) -> (Vec<ValueRange>, Vec<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        let (left, common, right) = self.source.left_common_right(value);
        if let Some(common) = common {
            mapped.push(ValueRange {
                start: self.destination - self.source.start + common.start,
                end: self.destination - self.source.start + common.end,
            });
            if let Some(left) = left {
                unmapped.push(left);
            }
            if let Some(right) = right {
                unmapped.push(right);
            }
            (unmapped, mapped)
        } else {
            (vec![value], vec![])
        }
    }
}
impl CategoryMapper for Vec<CategoryMap> {
    fn map(&self, value: ValueRange) -> (Vec<ValueRange>, Vec<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = vec![value];
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        for category_map in self {
            let (just_unmapped, mut just_mapped) = unmapped
                .iter()
                .map(|value_range| category_map.map(*value_range))
                .fold(
                    (Vec::<ValueRange>::new(), Vec::<ValueRange>::new()),
                    |(mut unmapped_acc, mut mapped_acc), (unmapped, mapped)| {
                        unmapped_acc.append(&mut unmapped.clone());
                        mapped_acc.append(&mut mapped.clone());
                        (unmapped, mapped)
                    },
                );
            unmapped = auto_subtract(just_unmapped);
            mapped.append(&mut just_mapped);
            mapped = auto_subtract(mapped);
        }
        (unmapped, mapped)
    }
}
impl CategoryMapper for Vec<Vec<CategoryMap>> {
    fn map(&self, value: ValueRange) -> (Vec<ValueRange>, Vec<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = vec![value];
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        unmapped.push(value);
        for category_map in self {
            let (mut just_unmapped, mut just_mapped) = unmapped
                .iter()
                .map(|value_range| category_map.map(*value_range))
                .fold(
                    (Vec::<ValueRange>::new(), Vec::<ValueRange>::new()),
                    |(mut unmapped_acc, mut mapped_acc), (unmapped, mapped)| {
                        unmapped_acc.append(&mut unmapped.clone());
                        mapped_acc.append(&mut mapped.clone());
                        (unmapped_acc, mapped_acc)
                    },
                );
            unmapped.clear();
            unmapped.append(&mut just_mapped.clone());
            unmapped.append(&mut just_unmapped.clone());
            unmapped = auto_subtract(unmapped);
        }

        (vec![], unmapped)
    }
}

pub fn seed_fertiliser_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut lines = reader.lines().filter_map(std::io::Result::ok);

    let seeds = ValueRange::from_vec(
        &mut lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .flat_map(str::parse::<i64>)
            .collect::<Vec<i64>>(),
    );

    let category_maps = lines.filter(|s| !s.is_empty()).fold(
        vec![] as Vec<Vec<CategoryMap>>,
        |mut acc: Vec<Vec<CategoryMap>>, line| -> Vec<Vec<CategoryMap>> {
            if line.contains("map") {
                acc.push(vec![]);
            } else {
                let last = acc.len() - 1;
                acc[last].push(CategoryMap::from_str(&line));
            }
            acc
        },
    );

    println!(
        "{:#?}",
        seeds
            .into_iter()
            .flat_map(|seed| category_maps.map(seed).1)
            .map(|range| range.start)
            .min()
    );

    /*
    let mut line_p = reader.lines().into_iter();
    let seeds = line_p.next();

    let _discard = line_p.next();
    let mut ranges: Vec<BTreeMap<i64, (i64, i64)>> = Vec::new();

    loop {
        let header = line_p.next().unwrap().unwrap();

        let mut curr_range: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
        // Inner Loop
        loop {
            let line = match line_p.next() {
                Some(line_ct) => match line_ct {
                    Ok(line_ct_2) => line_ct_2,
                    Err(e) => {
                        eprintln!("Error: {}", e.to_string());
                        break;
                    }
                },
                None => {
                    break;
                }
            };

            if line.len() == 0 {
                break;
            }
            let mut vals = line.split(" ").map(|x| x.trim().parse::<i64>().unwrap());

            let dest_start = vals.next().unwrap();
            let src_start = vals.next().unwrap();
            let range = vals.last().unwrap();
            //Start, Delta, Stop
            curr_range.insert(src_start, (dest_start - src_start, src_start + range));
        }

        ranges.push(curr_range);

        let dest = header.split(" ").into_iter().next().unwrap();
        if dest.ends_with("location") {
            break;
        }
    }

    // Start, Range
    let raw_seeds = seeds
        .unwrap()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .fold(vec![], |mut acc, e| {
            if acc.len() > 0 {
                let mut l: Vec<i64> = acc.pop().unwrap();
                if l.len() == 2 {
                    acc.push(l);
                    acc.push(vec![e]);
                } else {
                    l.push(e);
                    acc.push(l);
                }
            } else {
                acc.push(vec![e]);
            }
            acc
        });

    let out = raw_seeds.into_iter()
        .map(|seeds| {
            let range_start = seeds[0];
            let range_len = seeds[1];

            // Iterate through maps to get final range, and min

            let mut curr_range = BTreeMap::from([(range_start, range_len)]);

            for range_map in ranges.clone() {
                let mut fut_range = BTreeMap::new();
                for (r_s, r_l) in curr_range.into_iter() {
                    let r_e = r_s + r_l;
                    let mut tmp_ranges: Vec<(i64, i64)> = Vec::new();
                    for (src_start, (delta, src_end)) in range_map.clone() {
                        if src_end < r_s {
                            continue
                        }
                        if r_e < src_start {
                            // Traversed too far
                            break;
                        }
                        // There is an intersection
                        if r_s < src_start {
                            // Smaller bound
                            tmp_ranges.push((r_s, src_start - 1));
                        }

                        // Push intersecting range
                        tmp_ranges.push((max(r_s, src_start) + delta, min(src_end, r_e) + delta));

                        if r_e > src_end {
                            tmp_ranges.push((src_end + 1, r_e));
                        }
                    }
                    if tmp_ranges.len() == 0 {
                        tmp_ranges.push((r_s, r_e));
                    }
                    tmp_ranges.into_iter().for_each(|v| {
                        fut_range.insert(v.0, v.1);
                    })
                }
                curr_range = fut_range;
            }
            println!("Curr Range: {:?}", curr_range);
            let out = curr_range.keys().min().unwrap();
            *out
        }).collect::<Vec<i64>>();

        let out = out.into_iter().min().unwrap();

        println!("Out: {}", out);

        */
}
