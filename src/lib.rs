pub enum Category {
    Ones,
    Yatzy,
    Chance,
}
pub fn yatzy(category: Category, roll: Vec<u32>) -> u32 {
    match category {
        Category::Ones => 0,
        Category::Yatzy => score_yatzy(roll),
        Category::Chance => score_chance(roll),
    }
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