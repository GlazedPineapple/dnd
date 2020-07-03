use crate::roll::{Dice, Roll};
use anyhow::{anyhow, Context};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
pub struct CLIArgs {
    /// what to roll
    pub roll: Vec<String>,
    #[structopt(short, long)]
    /// whether player has advantage
    pub advantage: bool,
    #[structopt(short, long)]
    /// whether player has disadvantage
    pub disadvantage: bool,
    #[structopt(short, long)]
    /// whether to hide verbose messages from the output
    pub silent: bool,
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

        let mut roll = Roll {
            dice: Vec::with_capacity(captures.len()),
            modifier: None,
        };

        // Store the last end position as to find gaps in matches
        let mut last = 0;
        // Loop over matches
        for (i, cap) in captures.iter().enumerate() {
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

            if d == "d" {
                let count = number_at_beginning.parse().unwrap_or(1);
                let faces = number_at_end.parse().unwrap_or(6);

                if count == 0 {
                    return Err(anyhow!(
                        "`{}` is an invalid die. You must have one or more dice.",
                        whole.as_str()
                    ));
                }

                if faces == 0 {
                    return Err(anyhow!(
                        "`{}` is an invalid die. A die cannot have 0 faces.",
                        whole.as_str()
                    ));
                }

                roll.dice.push(Dice { count, faces });
            } else {
                if i != captures.len() - 1 {
                    return Err(anyhow!(
                        "The constant `{}` must be the last element in the dice",
                        whole.as_str()
                    ));
                }
                roll.modifier.replace(
                    format!("{}{}", number_at_beginning, number_at_end)
                        .parse()
                        .unwrap_or(0),
                );
            }

            // Update the position of the last matched
            last = whole.end();
        }

        // Check for unmatched die at the end of the string and report it
        if last != input.len() {
            return Err(anyhow!(
                "The dice were malformed between position '{}' and the end: {}",
                last,
                &input[last..input.len()]
            ));
        }

        Ok(roll)
    }
}
