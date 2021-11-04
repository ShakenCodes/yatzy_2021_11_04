pub enum Category {
    Chance,
    Yatzy,
}
pub fn yatzy(category: Category, roll: Vec<u32>) -> u32 {
    match category {
        Category::Chance => roll.into_iter().fold(0, |sum, x| sum + x),
        Category::Yatzy => 50,
    }
}