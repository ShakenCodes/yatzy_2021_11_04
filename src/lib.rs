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
    if not_two_groups(&roll) { return 0; }
    let roll_histogram = create_roll_histogram(&roll);
    if roll_histogram[&roll[0]] < 2 || roll_histogram[&roll[0]] > 3 { return 0; }
    sum_rolls(roll)
}
fn create_roll_histogram(roll: &Vec<u32>) -> HashMap<u32, u32> {
    roll.into_iter().fold(HashMap::new(), |mut h, x| { *h.entry(*x).or_insert(0) += 1; h } )
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
 fn not_two_groups(roll: &Vec<u32>) -> bool {
    create_roll_histogram(&roll).len() != 2
 }