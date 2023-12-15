// Camel Cards

use crate::utils::extract_file;
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandKind {
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 7,
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind = 6,
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse = 5,
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind = 4,
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair = 3,
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair = 2,
    // High card, where all cards' labels are distinct: 23456
    HighCard = 1,
}

fn get_ord_val(c: char) -> i8 {
    let vals = "AKQJT98765432".to_string();
    vals.chars()
        .rev()
        .position(|chr| chr == c)
        .map(|x| x as i8)
        .unwrap_or(-1)
}

fn comp_chars(c_1: char, c_2: char) -> std::cmp::Ordering {
    get_ord_val(c_1).cmp(&get_ord_val(c_2))
}

#[derive(Debug, Eq, PartialEq)]
struct CardBet {
    hand: String,
    kind: HandKind,
    bet: u32,
}

impl Ord for CardBet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = &self.hand;
        let other_val = &other.hand;

        let self_kind = &self.kind;
        let other_kind = &other.kind;

        match self_kind.cmp(&other_kind) {
            std::cmp::Ordering::Less => {
                return std::cmp::Ordering::Less;
            }
            std::cmp::Ordering::Greater => {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal => {
                let mut v = self_val
                    .chars()
                    .zip(other_val.chars())
                    .map(|(c_1, c_2)| comp_chars(c_1, c_2))
                    .filter(|&x| x != std::cmp::Ordering::Equal);

                match v.clone().count() {
                    0 => {
                        return std::cmp::Ordering::Equal;
                    }
                    _ => v.next().unwrap(),
                }
            }
        }
    }
}

impl CardBet {
    fn parse_from_string(s: String) -> Option<CardBet> {
        let parts: Vec<String> = s.split_whitespace().map(|x| x.to_string()).collect();

        if parts.len() != 2 {
            return None;
        }

        let hand = parts[0].clone();

        let vals = hand.chars().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e).or_insert(0) += 1;
            acc
        });

        let kind = match vals.len() {
            1 => HandKind::FiveOfAKind,
            2 => {
                let mut out: Option<HandKind> = None;
                for (_, count) in vals.clone() {
                    if count == 4 {
                        out = Some(HandKind::FourOfAKind);
                        break;
                    }
                }
                match out {
                    Some(val) => val,
                    None => HandKind::FullHouse,
                }
            }
            3 => {
                let mut out: Option<HandKind> = None;
                for (_, count) in vals.clone() {
                    if count == 3 {
                        out = Some(HandKind::ThreeOfAKind);
                        break;
                    }
                }
                match out {
                    Some(val) => val,
                    None => HandKind::TwoPair,
                }
            }
            4 => HandKind::OnePair,
            _ => HandKind::HighCard,
        };

        if let Ok(num) = parts[1].parse::<u32>() {
            return Some(CardBet {
                hand,
                kind,
                bet: num,
            });
        }
        None
    }

    fn get_bet(&self) -> u32 {
        self.bet
    }
}

impl PartialOrd for CardBet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn camel_cards(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader.lines().filter_map(std::io::Result::ok);

    let mut cards = lines
        .filter_map(CardBet::parse_from_string)
        .collect::<Vec<_>>();
    cards.sort();

    let card_sum = cards
        .into_iter()
        .enumerate()
        .map(|(index, bet)| (index as u32 + 1) * bet.get_bet())
        .sum::<u32>();

    println!("Sum: {}", card_sum);
}
