use anyhow::{anyhow, Context};
use structopt::StructOpt;
use regex::Regex;
use std::str::FromStr;

#[derive(StructOpt, PartialEq, Debug)]
/// A tool to roll DND dice with extra sex
pub struct CLIArgs {
    /// what to roll
    pub roll: Option<Roll>,
    /// whether player has advantage
    pub advantage: Option<Advantage>,
}

#[derive(PartialEq, Debug)]
pub struct Roll {
    /// The dice to roll
    pub dice: Vec<Dice>,
    /// The modifier to append to the result
    pub modifier: Option<u8>,
}

impl FromStr for Roll {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Compile the regex for use matching the die sequence
        let re = Regex::new(r"(?i)(?:^|(?:\s?\+\s?))([0-9]*)(d)?([0-9]+)")
            .context("Failed to compile regex for matching roll sequence")?;

        // Load the results from the input
        let results = re.find_iter(input);

        // Store the last end position as to find gaps in matches
        let mut last = 0;
        // Loop over matches
        for result in results {
            // Detect a gap in the matches and report it
            if last != result.start() {
                return Err(
                    anyhow!("The dice were malformed between positons '{}' and '{}': {}", last, result.start(), &input[last..result.start()]),
                );
            }

            // Update the position of the last matched
            last = result.end();

            // FIXME: Print out the result for debugging
            println!("result: {:?} {}", result, result.as_str());
        }

        // Check for unmatched die at the end of the string and report it
        if last != input.len() {
            return Err(anyhow!(
                "There is malformed data at the end of the dice string: {}", &input[last..input.len()]
            ));
        }

        Ok(Roll { dice: vec![], modifier: None })
        // todo!();
    }
}

#[derive(PartialEq, Debug)]
/// A group of like dice that will be rolled
pub struct Dice {
    /// The count of dice to roll
    pub count: u8,
    /// The faces on the die to roll
    pub faces: u8,
}

#[derive(PartialEq, Debug)]
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
