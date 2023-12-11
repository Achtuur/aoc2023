use std::{path::PathBuf, collections::HashMap};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day7 {
    input: String,
}

impl Day for Day7 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day7.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let sum = self.input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = Hand::from_str_A(split.next().unwrap());
            let points = split.next().unwrap().parse::<usize>().unwrap();
            (hand, points)
        })
        .sorted_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap())
        .enumerate()
        .map(|(i, (_, points))| {
            points * (i + 1)
        }).sum::<usize>();

        Ok(sum.to_string())
    }

    fn B(&mut self) -> Result<String> {
        let sum = self.input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = Hand::from_str_B(split.next().unwrap());
            let points = split.next().unwrap().parse::<usize>().unwrap();
            (hand, points)
        })
        .sorted_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap())
        .enumerate()
        .map(|(i, (hand, points))| {
            // println!("(i, hand): {0:?}", (i, hand, points));
            points * (i + 1)
        }).sum::<usize>();

        Ok(sum.to_string())
    }

    
}


#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King, 
    Ace,
}

impl Card {
    fn rank(&self) -> usize {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 0, //change this to 11 to fix part A
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }

    fn from_char(c: &char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}

#[derive(Debug, PartialEq)]
enum Hand {
    FullHouse(String),
    FiveOfAKind(String),
    FourOfAKind(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}
impl Hand {
    fn from_str_A(hand: &str) -> Self {
        let hand_map: HashMap<Card, i32> = hand.chars().fold(HashMap::new(), |mut map, card| {
            let c = Card::from_char(&card);
            match map.get(&c) {
                Some(count) => map.insert(c, count + 1),
                None => map.insert(c, 1),
            };
            map
        });

        let hand_sorted = hand_map.into_iter()
        .sorted_by(|(_, count1), (_, count2)| count2.partial_cmp(count1).unwrap())
        .collect_vec();

        let most = hand_sorted.first().unwrap_or(&(Card::Two, 0)).1;
        let second_most = hand_sorted.get(1).unwrap_or(&(Card::Two, 0)).1;

        if most == 3 && second_most == 2 {
            Hand::FullHouse(hand.to_owned())
        }
        else if most == 5 {
            Hand::FiveOfAKind(hand.to_owned())
        }
        else if most == 4 {
            Hand::FourOfAKind(hand.to_owned())
        }
        else if most == 3 {
            Hand::ThreeOfAKind(hand.to_owned())
        }
        else if most == 2 && second_most == 2 {
            Hand::TwoPair(hand.to_owned())
        }
        else if most == 2 {
            Hand::OnePair(hand.to_owned())
        } 
        else {
            Hand::HighCard(hand.to_owned())
        }
    }

    fn from_str_B(hand: &str) -> Self {
        let hand_map: HashMap<Card, i32> = hand.chars().fold(HashMap::new(), |mut map, card| {
            let c = Card::from_char(&card);
            match map.get(&c) {
                Some(count) => map.insert(c, count + 1),
                None => map.insert(c, 1),
            };
            map
        });

        let jokers = if let Some(count) = hand_map.iter().find(|(&card, _)| card == Card::Jack) {
            *count.1
        } else {
            0
        };

        let hand_sorted = hand_map.into_iter()
        .filter(|(card, _)| *card != Card::Jack)
        .sorted_by(|(_, count1), (_, count2)| count2.partial_cmp(count1).unwrap())
        .collect_vec();

        let most = hand_sorted.first().unwrap_or(&(Card::Two, 0)).1;
        let second_most = hand_sorted.get(1).unwrap_or(&(Card::Two, 0)).1;

        if jokers + most == 5 {
            Hand::FiveOfAKind(hand.to_owned())
        }
        else if jokers + most == 4 {
            Hand::FourOfAKind(hand.to_owned())
        }
        else if (0..=jokers).any(|n_j| n_j + most == 3 && (jokers - n_j) + second_most == 2) {
            Hand::FullHouse(hand.to_owned())
        }
        else if jokers + most == 3 {
            Hand::ThreeOfAKind(hand.to_owned())
        }
        else if (0..=jokers).any(|n_j| n_j + most == 2 && (jokers - n_j) + second_most == 2) {
            Hand::TwoPair(hand.to_owned())
        }
        else if jokers + most == 2 {
            Hand::OnePair(hand.to_owned())
        } 
        else {
            Hand::HighCard(hand.to_owned())
        }
    }

    fn to_num(&self) -> usize {
        match self {
            Hand::FiveOfAKind(_) => 6,
            Hand::FourOfAKind(_) => 5,
            Hand::FullHouse(_) => 4,
            Hand::ThreeOfAKind(_) => 3,
            Hand::TwoPair(_) => 2,
            Hand::OnePair(_) => 1,
            Hand::HighCard(_) => 0,
        }
    }

    fn raw(&self) -> &str {
        match self {
            Hand::FiveOfAKind(s) => s,
            Hand::FullHouse(s) => s,
            Hand::FourOfAKind(s) => s,
            Hand::ThreeOfAKind(s) => s,
            Hand::TwoPair(s) => s,
            Hand::OnePair(s) => s,
            Hand::HighCard(s) => s,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.to_num().partial_cmp(&other.to_num()) {
            Some(std::cmp::Ordering::Equal) => {
                for (c1, c2) in self.raw().chars().zip(other.raw().chars()) {
                    let (card1, card2) = (Card::from_char(&c1), Card::from_char(&c2));
                    match card1.partial_cmp(&card2) {
                        Some(std::cmp::Ordering::Equal) => (),
                        Some(ordering) => return Some(ordering),
                        None => return None,
                    };
                }
                None
            },
            Some(ordering) => Some(ordering),
            None => None,
        }
    }
}
