pub enum Category {
    Yatzy,
    Chance,
}
pub fn yatzy(category: Category, roll: Vec<u32>) -> u32 {
    match category {
        Category::Yatzy => score_yatzy(),
        Category::Chance => score_chance(roll),
    }
}

fn score_yatzy() -> u32 {
    50
}

fn score_chance(roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| sum + x)
}