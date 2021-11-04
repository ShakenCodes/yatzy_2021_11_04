pub enum Category {
    Ones,
    Yatzy,
    Chance,
}
pub fn yatzy(category: Category, roll: Vec<u32>) -> u32 {
    match category {
        Category::Ones => score_ones(roll),
        Category::Yatzy => score_yatzy(roll),
        Category::Chance => score_chance(roll),
    }
}

fn score_ones(roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| add_value_if_same(1, sum, x))
}
fn add_value_if_same(test_val: u32, sum: u32, value: u32) -> u32 {
    if test_val != value { return sum; }
    sum + value
}

fn score_yatzy(roll: Vec<u32>) -> u32 {
    if are_all_elements_equal(roll) {
        return 50;
    }
    0
}
fn are_all_elements_equal(roll: Vec<u32>) -> bool {
    roll.windows(2).all(|w| w[0] == w[1])
}

fn score_chance(roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| sum + x)
}