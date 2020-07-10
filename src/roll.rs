use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};

pub fn roll(roll: Roll) -> RollResult {
    let mut rng = thread_rng();

    // Total sum of the die rolls
    let mut sum = 0;
    let mut rolls = Vec::with_capacity(
        roll.dice
            .iter()
            .fold(0, |total, current| total + current.count as usize),
    );
    for die in roll.dice {
        for _ in 0..die.count {
            let result = rng.gen_range(1, die.faces + 1);
            sum += result;
            rolls.push(DieRoll {
                result,
                faces: die.faces,
                sum,
            });
        }
    }

    RollResult {
        sum: sum + roll.modifier.unwrap_or(0),
        rolls,
        modifier: roll.modifier,
    }
}

#[derive(Debug)]
// The result of a roll
pub struct RollResult {
    pub sum: u32,
    pub rolls: Vec<DieRoll>,
    pub modifier: Option<u32>,
}

#[derive(Debug)]
// The result of an individual die roll
pub struct DieRoll {
    pub result: u32,
    pub faces: u32,
    pub sum: u32,
}

pub fn roll_with_advantage(
    faces: u32,
    advantage: &Advantage,
    modifier: Option<u32>,
) -> AdvantageRollResult {
    let mut rng = thread_rng();

    let a = rng.gen_range(1, faces + 1);
    let b = rng.gen_range(1, faces + 1);

    let result_sin_modifier = match advantage {
        Advantage::Advantage => a.max(b),
        Advantage::Disadvantage => a.min(b),
    };

    AdvantageRollResult {
        result: result_sin_modifier + modifier.unwrap_or(0),
        result_sin_modifier,
        a,
        b,
    }
}

#[derive(Debug)]
/// The results of a roll with (dis)advantage
pub struct AdvantageRollResult {
    pub a: u32,
    pub b: u32,
    pub result: u32,
    pub result_sin_modifier: u32,
}

#[derive(Debug)]
pub struct Roll {
    /// The dice to roll
    pub dice: Vec<Dice>,
    /// The modifier to append to the result
    pub modifier: Option<u32>,
}

#[derive(Debug)]
/// A group of like dice that will be rolled
pub struct Dice {
    /// The count of dice to roll
    pub count: u32,
    /// The faces on the die to roll
    pub faces: u32,
}

#[derive(Debug)]
/// The advantage or lack thereof that multiple rolls will have
pub enum Advantage {
    Advantage,
    Disadvantage,
}

impl Display for Advantage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Advantage::Advantage => write!(f, "advantage"),
            Advantage::Disadvantage => write!(f, "disadvantage"),
        }
    }
}
