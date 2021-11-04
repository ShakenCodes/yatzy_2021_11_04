pub fn yatzy(roll: Vec<u32>) -> u32 {
    roll.into_iter().fold(0, |sum, x| sum + x)
}