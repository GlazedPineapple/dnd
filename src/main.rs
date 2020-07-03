use anyhow::anyhow;
use args::CLIArgs;
use colored::*;
use roll::{Advantage, Dice, Roll};
use std::str::FromStr;
use structopt::StructOpt;

mod args;
mod roll;

fn main() -> anyhow::Result<()> {
    let args = CLIArgs::from_args();

    let roll = if args.roll.is_empty() {
        None
    } else {
        Some(Roll::from_str(&args.roll.join(""))?)
    };

    let roll = roll.unwrap_or_else(|| {
        eprintln!("No dice were provided, defaulting to 1d20");

        Roll {
            dice: vec![Dice {
                count: 1,
                faces: 20,
            }],
            modifier: None,
        }
    });

    let advantage = match (args.advantage, args.disadvantage) {
        (false, false) => None,
        (false, true) => Some(Advantage::Disadvantage),
        (true, false) => Some(Advantage::Advantage),
        (true, true) => {
            return Err(anyhow!(
                "You cannot have both advantage and disadvantage on one roll"
            ))
        }
    };

    if let Some(advantage) = advantage {
        if roll.dice.is_empty() {
            return Err(anyhow!("If you get this error you fucked up pretty bad"));
        }

        if roll.dice.len() > 1 || roll.dice[0].count > 1 {
            return Err(anyhow!(
                "You cannot have more then one dice when using advantage/disadvantage"
            ));
        }

        let result = roll::roll_with_advantage(roll.dice[0].faces, &advantage, roll.modifier);

        if args.silent {
            println!("{}", result.result);
        } else {
            print!(
                "2 d{}s were rolled with the values of {} and {}. Since they were rolled with {}, the {} value is ",
                roll.dice[0].faces,
                result.a,
                result.b,
                advantage,
                match advantage{
                    Advantage::Advantage => "higher",
                    Advantage::Disadvantage => "lower",
                }
            );

            if let Some(modifier) = roll.modifier {
                print!(
                    "{} plus the modifier {} is ",
                    result.result_sin_modifier, modifier
                );
            }
            println!("{}.", result.result.to_string().green().bold());
        }
    } else {
        let result = roll::roll(roll);

        if args.silent {
            println!("{}", result.sum);
        } else {
            for roll in result.rolls {
                println!(
                    "A d{} was rolled with the result {}, bringing the current sum to {}",
                    roll.faces, roll.result, roll.sum
                );
            }
            if let Some(modifier) = result.modifier {
                println!(
                    "A modifier of {} was added bringing the total sum to {}",
                    modifier, result.sum.to_string().green().bold()
                );
            } else {
                println!("The total sum is {}", result.sum.to_string().green().bold());
            }
        }
        // dbg!(result);
    };

    Ok(())
}
