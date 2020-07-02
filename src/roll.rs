use rand::{thread_rng, Rng};

pub fn roll(roll: Roll, advantage: Option<Advantage>) -> u32 {
    let mut rng = thread_rng();

    let roll: u32 = rng.gen_range(1, 10);
}

#[derive(Debug)]
pub struct Roll {
    /// The dice to roll
    pub dice: Vec<Dice>,
    /// The modifier to append to the result
    pub modifier: Option<u8>,
}

#[derive(Debug)]
/// A group of like dice that will be rolled
pub struct Dice {
    /// The count of dice to roll
    pub count: u8,
    /// The faces on the die to roll
    pub faces: u8,
}

#[derive(Debug)]
/// The advantage or lack thereof that multiple rolls will have
pub enum Advantage {
    Advantage,
    Disadvantage,
}
