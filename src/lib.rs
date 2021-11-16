use std::{collections::HashMap};

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Yatzy,
    Chance,
}
pub fn yatzy(category: Category, roll: Vec<u32>) -> u32 {
    match category {
        Category::Ones => score_uppers(1, roll),
        Category::Twos => score_uppers(2, roll),
        Category::Threes => score_uppers(3, roll),
        Category::Fours => score_uppers(4, roll),
        Category::Fives => score_uppers(5, roll),
        Category::Sixes => score_uppers(6, roll),
        Category::FullHouse => score_full_house(roll),
        Category::SmallStraight => score_small_straight(roll),
        Category::LargeStraight => score_large_straight(roll),
        Category::Yatzy => score_yatzy(roll),
        Category::Chance => score_chance(roll),
    }
}

fn score_uppers(category_value: u32, roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| add_value_if_same(category_value, sum, x))
}
fn add_value_if_same(test_val: u32, sum: u32, value: u32) -> u32 {
    if test_val != value { return sum; }
    sum + value
}

fn score_full_house(roll: Vec<u32>) -> u32 {
    if two_of_some_die(&roll) && three_of_some_die(&roll) { return sum_rolls(roll); }
    0
}

fn score_small_straight(roll: Vec<u32>) -> u32 {
    if roll == vec![1,2,3,4,5] { return 15; }
    0
}

fn score_large_straight(roll: Vec<u32>) -> u32 {
    if roll == vec![2,3,4,5,6] { return 20; }
    0
}

fn score_yatzy(roll: Vec<u32>) -> u32 {
    if are_all_elements_equal(roll) { return 50; }
    0
}
fn are_all_elements_equal(roll: Vec<u32>) -> bool {
    roll.windows(2).all(|w| w[0] == w[1])
}

fn score_chance(roll: Vec<u32>) -> u32 {
    sum_rolls(roll)
}

fn sum_rolls(roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| sum + x)
}

fn two_of_some_die(roll: &Vec<u32>) -> bool {
    let mut twice_rolled_histogram = create_roll_histogram(&roll);
    twice_rolled_histogram.retain(|_, v| *v == 2);
    twice_rolled_histogram.len() > 0
 }
 fn three_of_some_die(roll: &Vec<u32>) -> bool {
    let mut thrice_rolled_histogram = create_roll_histogram(&roll);
    thrice_rolled_histogram.retain(|_, v| *v == 3);
    thrice_rolled_histogram.len() > 0
 }
 
fn create_roll_histogram(roll: &Vec<u32>) -> HashMap<u32, u32> {
    roll.into_iter().fold(HashMap::new(), |mut h, x| { *h.entry(*x).or_insert(0) += 1; h } )
}
