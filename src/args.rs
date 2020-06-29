use anyhow::{anyhow, Context};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
pub struct CLIArgs {
    /// what to roll
    pub roll: Option<Roll>,
    /// whether player has advantage
    pub advantage: Option<Advantage>,
}

#[derive(Debug)]
pub struct Roll {
    /// The dice to roll
    pub dice: Vec<Dice>,
    /// The modifier to append to the result
    pub modifier: Option<u8>,
}

impl FromStr for Roll {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_lowercase();

        // Compile the regex for use matching the die sequence
        let re = Regex::new(r"(?i)(?:^|(?:\s?\+\s?))([0-9]*)(d?)([0-9]+)")
            .context("Failed to compile regex for matching roll sequence")?;

        // Load the captures from the input
        let captures: Vec<_> = re.captures_iter(&input).collect();

        let roll = Roll {
            dice: Vec::with_capacity(captures.len()),
            modifier: None,
        };

        // Store the last end position as to find gaps in matches
        let mut last = 0;
        // Loop over matches
        for cap in captures {
            let whole = cap.get(0).unwrap();

            let number_at_beginning = cap.get(1).unwrap().as_str();
            let d = cap.get(2).unwrap().as_str();
            let number_at_end = cap.get(3).unwrap().as_str();

            // Detect a gap in the matches and report it
            if last != whole.start() {
                return Err(anyhow!(
                    "The dice were malformed between positons '{}' and '{}': {}",
                    last,
                    whole.start(),
                    &input[last..whole.start()]
                ));
            }

            // Update the position of the last matched
            last = whole.end();

            if d == "d" {
                println!("Die");
            } else {
                println!("const");
            }

            // FIXME: Print out the result for debugging
            println!("{} | {} | {} |||| {}", number_at_beginning, d, number_at_end, whole.as_str());
        }

        // Check for unmatched die at the end of the string and report it
        if last != input.len() {
            return Err(anyhow!(
                "The dice were malformed between position '{}' and the end: {}",
                last,
                &input[last..input.len()]
            ));
        }

        println!("_________");

        Ok(roll)
    }
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

impl FromStr for Advantage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "advantage" | "a" => Ok(Self::Advantage),
            "disadvantage" | "d" => Ok(Self::Disadvantage),
            _ => Err(anyhow!("The advantage type provided was not valid. Only `advantage` or `disadvantage` or their first letters are valid.")),
        }
    }
}
